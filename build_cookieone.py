#!/usr/bin/env python3
"""
CookieOne Cross-Platform Build Script with Beautiful GUI
========================================================

A comprehensive build automation script with a modern, animated interface
for building the CookieOne offline-first voice assistant project.

Requirements:
    - Python 3.7+
    - pip install rich click colorama psutil

Usage:
    python build_cookieone.py [OPTIONS]

Author: CookieOne Build System
License: GPL-3.0
"""

import os
import sys
import time
import shutil
import subprocess
import platform
import json
import zipfile
import threading
from datetime import datetime, timedelta
from pathlib import Path
from typing import List, Optional, Dict, Tuple, Any
from dataclasses import dataclass, field
from enum import Enum
from queue import Queue, Empty

try:
    import click
    from rich.console import Console
    from rich.layout import Layout
    from rich.panel import Panel
    from rich.live import Live
    from rich.table import Table
    from rich.text import Text
    from rich.align import Align
    from rich import box
    import colorama
    import psutil
except ImportError as e:
    missing_parts = str(e).split("'")
    missing_pkg = missing_parts[1] if len(missing_parts) > 1 else str(e)
    print(f"❌ Missing required dependency: {missing_pkg}")
    print("\nPlease install the build dependencies before running this script.")
    print("Suggested setup:")
    print("  python3 -m venv .venv")
    print("  source .venv/bin/activate  # On Windows: .venv\\Scripts\\activate")
    print("  pip install -r requirements-build.txt")
    print("\nAlternatively install manually: pip install rich click colorama psutil")
    sys.exit(1)

# Initialize colorama for cross-platform color support
colorama.init(autoreset=True)

# Constants
VERSION = "1.0.0"
PROJECT_NAME = "CookieOne Voice Assistant"
DEFAULT_BRANCH = "final-release-wakeword-windows-build-cargo-check"

# Unicode symbols for beautiful UI
SYMBOLS = {
    'check': '✓',
    'cross': '✗',
    'arrow': '→',
    'bullet': '•',
    'star': '★',
    'heart': '♥',
    'circle': '●',
    'square': '■',
    'diamond': '◆',
    'hourglass': '⌛',
    'rocket': '🚀',
    'wrench': '🔧',
    'package': '📦',
    'fire': '🔥',
    'sparkles': '✨',
    'warning': '⚠',
    'info': 'ℹ',
    'lightning': '⚡',
}


class BuildStatus(Enum):
    """Build stage status enumeration"""
    PENDING = ("⏳", "dim white", "Pending")
    RUNNING = ("🔄", "bold cyan", "Running")
    SUCCESS = ("✅", "bold green", "Success")
    FAILED = ("❌", "bold red", "Failed")
    SKIPPED = ("⏭", "yellow", "Skipped")
    PAUSED = ("⏸", "bold yellow", "Paused")


@dataclass
class BuildStage:
    """Represents a single build stage"""
    name: str
    description: str
    command: Optional[str] = None
    status: BuildStatus = BuildStatus.PENDING
    start_time: Optional[float] = None
    end_time: Optional[float] = None
    error_message: Optional[str] = None
    
    @property
    def duration(self) -> float:
        """Calculate stage duration in seconds"""
        if self.start_time is None:
            return 0.0
        end = self.end_time if self.end_time else time.time()
        return end - self.start_time
    
    @property
    def duration_str(self) -> str:
        """Format duration as human-readable string"""
        duration = self.duration
        if duration < 60:
            return f"{duration:.1f}s"
        elif duration < 3600:
            minutes = int(duration // 60)
            seconds = int(duration % 60)
            return f"{minutes}m {seconds}s"
        else:
            hours = int(duration // 3600)
            minutes = int((duration % 3600) // 60)
            return f"{hours}h {minutes}m"


@dataclass
class BuildConfig:
    """Build configuration"""
    branch: str = DEFAULT_BRANCH
    release_mode: bool = False
    clean: bool = False
    no_gui: bool = False
    skip_tests: bool = False
    output_dir: Optional[Path] = None
    verbose: bool = False
    project_dir: Path = field(default_factory=lambda: Path.cwd())
    log_file: Optional[Path] = None


class LogCapture:
    """Captures and queues log output from subprocesses"""
    def __init__(self, queue: Queue):
        self.queue = queue
    
    def write(self, text: str):
        if text.strip():
            self.queue.put(text.strip())
    
    def flush(self):
        pass


class BuildController:
    """Main build controller with UI management"""
    
    def __init__(self, config: BuildConfig):
        self.config = config
        self.console = Console()
        self.stages: List[BuildStage] = []
        self.current_stage_idx: int = -1
        self.start_time: Optional[float] = None
        self.end_time: Optional[float] = None
        self.paused = False
        self.cancelled = False
        self.log_queue = Queue()
        self.log_buffer: List[str] = []
        self.max_log_lines = 200 if self.config.verbose else 100
        self.spinner_frames = ['⠋', '⠙', '⠹', '⠸', '⠼', '⠴', '⠦', '⠧', '⠇', '⠏']
        self.spinner_idx = 0
        self.pulse_state = 0
        self.start_event = threading.Event()
        self.pause_event = threading.Event()
        self.pause_event.set()
        self.controls_enabled = sys.stdin.isatty()
        if self.controls_enabled:
            self.control_thread = threading.Thread(target=self._control_loop, daemon=True)
            self.control_thread.start()
            self.auto_start_timer = threading.Timer(1.0, self.start_event.set)
            self.auto_start_timer.start()
        else:
            self.control_thread = None
            self.auto_start_timer = None
            self.start_event.set()
        
        # Setup logging
        self._setup_logging()
        
        # Initialize build stages
        self._init_stages()
    
    def _setup_logging(self):
        """Setup file logging"""
        if not self.config.log_file:
            log_dir = self.config.project_dir / "build_logs"
            log_dir.mkdir(exist_ok=True)
            timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
            self.config.log_file = log_dir / f"build_{timestamp}.log"
        
        self.log_file = open(self.config.log_file, 'w', encoding='utf-8')
        self._log_message(f"Build started at {datetime.now().isoformat()}")
        self._log_message(f"Configuration: {self.config}")
    
    def _log_message(self, message: str):
        """Write message to log file"""
        timestamp = datetime.now().strftime("%Y-%m-%d %H:%M:%S")
        self.log_file.write(f"[{timestamp}] {message}\n")
        self.log_file.flush()
    
    def _control_loop(self):
        """Background thread for control handling"""
        if not self.controls_enabled:
            return
        controls_hint = "Controls: [s]tart, [p]ause, [r]esume, [c]ancel, [e]xport report, [h]elp"
        self.log_queue.put(controls_hint)
        while True:
            if self.end_time is not None:
                break
            try:
                user_input = input().strip().lower()
            except EOFError:
                break
            except Exception:
                continue
            if not user_input:
                continue
            if user_input in ("start", "s"):
                if not self.start_event.is_set():
                    self.start_event.set()
                    self.log_queue.put("▶ Start command received.")
                else:
                    self.log_queue.put("ℹ Build already running.")
            elif user_input in ("pause", "p"):
                if self.pause_event.is_set():
                    self.pause_event.clear()
                    self.paused = True
                    self.log_queue.put("⏸ Build paused. Current stage will finish before pausing.")
                else:
                    self.log_queue.put("ℹ Build already paused.")
            elif user_input in ("resume", "r"):
                if not self.pause_event.is_set():
                    self.pause_event.set()
                    self.paused = False
                    self.log_queue.put("▶ Build resumed.")
                else:
                    self.log_queue.put("ℹ Build already running.")
            elif user_input in ("cancel", "c", "stop"):
                if not self.cancelled:
                    self.cancelled = True
                    self.pause_event.set()
                    self.start_event.set()
                    self.log_queue.put("🛑 Cancellation requested. Attempting graceful shutdown.")
                else:
                    self.log_queue.put("ℹ Cancellation already requested.")
            elif user_input in ("report", "e"):
                try:
                    report_path = self.generate_error_report()
                    self.log_queue.put(f"📁 Error report saved to {report_path}")
                except Exception as exc:
                    self.log_queue.put(f"⚠ Failed to generate error report: {exc}")
            elif user_input in ("help", "h"):
                self.log_queue.put(controls_hint)
            else:
                self.log_queue.put(f"⚠ Unknown command: {user_input}")
        self.log_queue.put("ℹ Control interface closed.")
    
    def _init_stages(self):
        """Initialize all build stages"""
        self.stages = [
            BuildStage(
                name="Environment Check",
                description="Checking build dependencies and environment",
                command=None  # Custom check
            ),
            BuildStage(
                name="Repository Preparation",
                description=f"Ensuring branch {self.config.branch}",
                command=None  # Custom git operations
            ),
            BuildStage(
                name="Clean Build Artifacts",
                description="Removing previous build artifacts",
                command=None  # Custom clean
            ) if self.config.clean else None,
            BuildStage(
                name="Build Rust Core (/app)",
                description="Building main Rust application crate",
                command=f"cargo build {'--release' if self.config.release_mode else ''}"
            ),
            BuildStage(
                name="Run Tests",
                description="Running Rust test suite",
                command="cargo test"
            ) if not self.config.skip_tests else None,
            BuildStage(
                name="Build Frontend",
                description="Building Svelte/Vite frontend",
                command=None  # Custom npm build
            ) if not self.config.no_gui else None,
            BuildStage(
                name="Build Tauri Application",
                description="Building Tauri desktop application",
                command=None  # Custom tauri build
            ) if not self.config.no_gui else None,
            BuildStage(
                name="Package Distribution",
                description="Creating final distribution package",
                command=None  # Custom packaging
            ),
            BuildStage(
                name="Finalization",
                description="Final cleanup and verification",
                command=None  # Custom finalization
            ),
        ]
        
        # Remove None stages (skipped stages)
        self.stages = [s for s in self.stages if s is not None]
    
    def _check_command_exists(self, command: str) -> bool:
        """Check if a command exists in PATH"""
        try:
            if platform.system() == "Windows":
                subprocess.run(['where', command], 
                             stdout=subprocess.PIPE, 
                             stderr=subprocess.PIPE,
                             check=True)
            else:
                subprocess.run(['which', command], 
                             stdout=subprocess.PIPE, 
                             stderr=subprocess.PIPE,
                             check=True)
            return True
        except subprocess.CalledProcessError:
            return False
    
    def _run_command(self, command: str, cwd: Optional[Path] = None, 
                     env: Optional[Dict[str, str]] = None) -> Tuple[int, str, str]:
        """Run a shell command and capture output"""
        self._log_message(f"Running command: {command} (cwd: {cwd})")
        
        if cwd is None:
            cwd = self.config.project_dir
        
        # Prepare environment
        cmd_env = os.environ.copy()
        if env:
            cmd_env.update(env)
        
        # Platform-specific command execution
        if platform.system() == "Windows":
            shell_cmd = command
        else:
            shell_cmd = command
        
        try:
            process = subprocess.Popen(
                shell_cmd,
                shell=True,
                cwd=str(cwd),
                env=cmd_env,
                stdout=subprocess.PIPE,
                stderr=subprocess.PIPE,
                text=True,
                bufsize=1
            )
            
            stdout_lines = []
            stderr_lines = []
            
            # Read output in real-time
            while True:
                if process.poll() is not None:
                    break
                if self.cancelled:
                    try:
                        process.terminate()
                    except Exception:
                        pass
                    self.log_queue.put("🛑 Command cancelled by user.")
                    self._log_message("Command cancelled by user.")
                    stdout_lines.append("Command cancelled by user.")
                    stderr_lines.append("Command cancelled by user.")
                    try:
                        process.wait(timeout=5)
                    except Exception:
                        pass
                    return 130, '\n'.join(stdout_lines), '\n'.join(stderr_lines)
                if not self.pause_event.is_set():
                    time.sleep(0.1)
                    continue
                
                # Read stdout
                if process.stdout:
                    line = process.stdout.readline()
                    if line:
                        line = line.rstrip()
                        stdout_lines.append(line)
                        self.log_queue.put(line)
                        self._log_message(f"[STDOUT] {line}")
                
                time.sleep(0.01)
            
            # Get remaining output
            if process.stdout:
                remaining = process.stdout.read()
                if remaining:
                    for line in remaining.split('\n'):
                        if line.strip():
                            stdout_lines.append(line.rstrip())
                            self.log_queue.put(line.rstrip())
                            self._log_message(f"[STDOUT] {line}")
            
            if process.stderr:
                stderr = process.stderr.read()
                if stderr:
                    for line in stderr.split('\n'):
                        if line.strip():
                            stderr_lines.append(line.rstrip())
                            self.log_queue.put(f"[ERROR] {line.rstrip()}")
                            self._log_message(f"[STDERR] {line}")
            
            returncode = process.returncode
            stdout = '\n'.join(stdout_lines)
            stderr = '\n'.join(stderr_lines)
            
            self._log_message(f"Command exit code: {returncode}")
            return returncode, stdout, stderr
            
        except Exception as e:
            error_msg = f"Command execution failed: {str(e)}"
            self._log_message(f"[ERROR] {error_msg}")
            self.log_queue.put(f"[ERROR] {error_msg}")
            return 1, "", error_msg
    
    def _check_environment(self) -> bool:
        """Check all required dependencies"""
        self.log_queue.put("🔍 Checking build environment...")
        
        required_tools = {
            'rust': 'rustc',
            'cargo': 'cargo',
        }
        
        if not self.config.no_gui:
            required_tools.update({
                'node': 'node',
                'npm': 'npm',
            })
        
        missing = []
        for name, cmd in required_tools.items():
            self.log_queue.put(f"  Checking {name}...")
            if self._check_command_exists(cmd):
                self.log_queue.put(f"    ✓ {name} found")
            else:
                self.log_queue.put(f"    ✗ {name} NOT FOUND")
                missing.append(name)
        
        if missing:
            error_msg = f"Missing required tools: {', '.join(missing)}"
            self.log_queue.put(f"\n❌ {error_msg}")
            self.log_queue.put("\nPlease install:")
            for tool in missing:
                if tool in ['rust', 'cargo']:
                    self.log_queue.put("  - Rust: https://rustup.rs/")
                elif tool in ['node', 'npm']:
                    self.log_queue.put("  - Node.js: https://nodejs.org/")
            return False
        
        # Check system resources
        memory = psutil.virtual_memory()
        disk = psutil.disk_usage(str(self.config.project_dir))
        
        self.log_queue.put(f"\n📊 System Resources:")
        self.log_queue.put(f"  CPU Cores: {psutil.cpu_count(logical=True)} ({psutil.cpu_count(logical=False)} physical)")
        self.log_queue.put(f"  RAM: {memory.total / (1024**3):.1f} GB ({memory.percent}% used)")
        self.log_queue.put(f"  Disk Space: {disk.free / (1024**3):.1f} GB free")
        
        if disk.free < 5 * (1024**3):  # Less than 5GB
            self.log_queue.put("  ⚠ Warning: Low disk space (< 5GB)")
        
        self.log_queue.put(f"\n✅ Environment check passed!")
        return True
    
    def _prepare_repository(self) -> bool:
        """Prepare git repository"""
        self.log_queue.put(f"📂 Preparing repository on branch '{self.config.branch}'...")
        
        # Check if we're in a git repo
        if not (self.config.project_dir / ".git").exists():
            self.log_queue.put("⚠ Not a git repository, skipping branch checkout")
            return True
        
        # Get current branch
        returncode, stdout, stderr = self._run_command("git rev-parse --abbrev-ref HEAD")
        if returncode == 0:
            current_branch = stdout.strip()
            self.log_queue.put(f"  Current branch: {current_branch}")
            
            if current_branch != self.config.branch:
                self.log_queue.put(f"  Switching to branch '{self.config.branch}'...")
                returncode, stdout, stderr = self._run_command(f"git checkout {self.config.branch}")
                if returncode != 0:
                    self.log_queue.put(f"  ⚠ Could not checkout branch: {stderr}")
                    self.log_queue.put(f"  Continuing with current branch...")
                else:
                    self.log_queue.put(f"  ✓ Switched to {self.config.branch}")
        
        # Update submodules if any
        self.log_queue.put("  Updating submodules...")
        self._run_command("git submodule update --init --recursive")
        
        return True
    
    def _clean_artifacts(self) -> bool:
        """Clean build artifacts"""
        self.log_queue.put("🧹 Cleaning build artifacts...")
        
        # Clean Rust target directories
        rust_target = self.config.project_dir / "app" / "target"
        if rust_target.exists():
            self.log_queue.put(f"  Removing {rust_target}...")
            try:
                shutil.rmtree(rust_target)
                self.log_queue.put("  ✓ Cleaned Rust artifacts")
            except Exception as e:
                self.log_queue.put(f"  ⚠ Failed to clean Rust artifacts: {e}")
        
        if not self.config.no_gui:
            # Clean frontend artifacts
            gui_dist = self.config.project_dir / "gui" / "dist"
            gui_node_modules = self.config.project_dir / "gui" / "node_modules"
            
            if gui_dist.exists():
                self.log_queue.put(f"  Removing {gui_dist}...")
                try:
                    shutil.rmtree(gui_dist)
                    self.log_queue.put("  ✓ Cleaned frontend dist")
                except Exception as e:
                    self.log_queue.put(f"  ⚠ Failed to clean frontend dist: {e}")
            
            # Clean Tauri target
            tauri_target = self.config.project_dir / "gui" / "src-tauri" / "target"
            if tauri_target.exists():
                self.log_queue.put(f"  Removing {tauri_target}...")
                try:
                    shutil.rmtree(tauri_target)
                    self.log_queue.put("  ✓ Cleaned Tauri artifacts")
                except Exception as e:
                    self.log_queue.put(f"  ⚠ Failed to clean Tauri artifacts: {e}")
        
        self.log_queue.put("✅ Cleanup complete")
        return True
    
    def _build_rust_app(self) -> bool:
        """Build Rust application"""
        self.log_queue.put("🦀 Building Rust application...")
        
        app_dir = self.config.project_dir / "app"
        if not app_dir.exists():
            self.log_queue.put("❌ /app directory not found")
            return False
        
        build_cmd = "cargo build"
        if self.config.release_mode:
            build_cmd += " --release"
            self.log_queue.put("  Using release mode (optimized)")
        
        returncode, stdout, stderr = self._run_command(build_cmd, cwd=app_dir)
        
        if returncode == 0:
            self.log_queue.put("✅ Rust build successful")
            return True
        else:
            self.log_queue.put(f"❌ Rust build failed")
            return False
    
    def _run_tests(self) -> bool:
        """Run test suite"""
        self.log_queue.put("🧪 Running tests...")
        
        app_dir = self.config.project_dir / "app"
        returncode, stdout, stderr = self._run_command("cargo test", cwd=app_dir)
        
        if returncode == 0:
            self.log_queue.put("✅ All tests passed")
            return True
        else:
            self.log_queue.put(f"❌ Tests failed")
            return False
    
    def _build_frontend(self) -> bool:
        """Build frontend"""
        self.log_queue.put("⚛️ Building frontend...")
        
        gui_dir = self.config.project_dir / "gui"
        if not gui_dir.exists():
            self.log_queue.put("❌ /gui directory not found")
            return False
        
        # Install dependencies
        self.log_queue.put("  Installing npm dependencies...")
        returncode, stdout, stderr = self._run_command("npm install", cwd=gui_dir)
        if returncode != 0:
            self.log_queue.put("❌ npm install failed")
            return False
        
        # Build
        self.log_queue.put("  Building frontend with Vite...")
        returncode, stdout, stderr = self._run_command("npm run build", cwd=gui_dir)
        
        if returncode == 0:
            self.log_queue.put("✅ Frontend build successful")
            return True
        else:
            self.log_queue.put("❌ Frontend build failed")
            return False
    
    def _build_tauri(self) -> bool:
        """Build Tauri application"""
        self.log_queue.put("🖥️ Building Tauri application...")
        
        gui_dir = self.config.project_dir / "gui"
        
        # Build Tauri
        self.log_queue.put("  Building Tauri bundle...")
        build_cmd = "npm run tauri build"
        if not self.config.release_mode:
            # For debug builds, use tauri dev build
            build_cmd = "npm run tauri build -- --debug"
        
        returncode, stdout, stderr = self._run_command(build_cmd, cwd=gui_dir)
        
        if returncode == 0:
            self.log_queue.put("✅ Tauri build successful")
            return True
        else:
            self.log_queue.put("❌ Tauri build failed")
            return False
    
    def _package_distribution(self) -> bool:
        """Package final distribution"""
        self.log_queue.put("📦 Packaging distribution...")
        
        # Determine output directory
        if not self.config.output_dir:
            self.config.output_dir = self.config.project_dir / "dist"
        
        self.config.output_dir.mkdir(parents=True, exist_ok=True)
        
        # Find Tauri bundles
        if not self.config.no_gui:
            tauri_target = self.config.project_dir / "gui" / "src-tauri" / "target"
            build_type = "release" if self.config.release_mode else "debug"
            bundle_dir = tauri_target / build_type / "bundle"
            
            if bundle_dir.exists():
                self.log_queue.put(f"  Copying bundles from {bundle_dir}...")
                
                # Copy all bundle formats
                for item in bundle_dir.iterdir():
                    if item.is_dir() or item.is_file():
                        dest = self.config.output_dir / item.name
                        if item.is_dir():
                            if dest.exists():
                                shutil.rmtree(dest)
                            shutil.copytree(item, dest)
                            self.log_queue.put(f"    ✓ Copied {item.name}/")
                        else:
                            shutil.copy2(item, dest)
                            self.log_queue.put(f"    ✓ Copied {item.name}")
            else:
                self.log_queue.put(f"  ⚠ Bundle directory not found: {bundle_dir}")
        
        # Copy standalone Rust binary if exists
        app_target = self.config.project_dir / "app" / "target"
        build_type = "release" if self.config.release_mode else "debug"
        binary_dir = app_target / build_type
        
        if binary_dir.exists():
            # Find executable
            exe_name = "jarvis-app" if platform.system() != "Windows" else "jarvis-app.exe"
            exe_path = binary_dir / exe_name
            
            if exe_path.exists():
                dest = self.config.output_dir / exe_name
                shutil.copy2(exe_path, dest)
                self.log_queue.put(f"  ✓ Copied standalone binary: {exe_name}")
        
        self.log_queue.put(f"✅ Distribution packaged in: {self.config.output_dir}")
        return True
    
    def _finalize(self) -> bool:
        """Finalize build"""
        self.log_queue.put("🎉 Finalizing build...")
        
        # Generate build manifest
        manifest = {
            'project': PROJECT_NAME,
            'version': VERSION,
            'build_time': datetime.now().isoformat(),
            'configuration': {
                'branch': self.config.branch,
                'release_mode': self.config.release_mode,
                'platform': platform.system(),
                'architecture': platform.machine(),
            },
            'stages': [
                {
                    'name': stage.name,
                    'status': stage.status.name,
                    'duration': stage.duration_str
                }
                for stage in self.stages
            ]
        }
        
        manifest_path = self.config.output_dir / "build_manifest.json"
        with open(manifest_path, 'w') as f:
            json.dump(manifest, f, indent=2)
        
        self.log_queue.put(f"  ✓ Build manifest: {manifest_path}")
        self.log_queue.put("✅ Build finalized")
        return True
    
    def _execute_stage(self, stage: BuildStage) -> bool:
        """Execute a single build stage"""
        stage.status = BuildStatus.RUNNING
        stage.start_time = time.time()
        
        success = False
        
        try:
            # Execute stage-specific logic
            if stage.name == "Environment Check":
                success = self._check_environment()
            elif stage.name == "Repository Preparation":
                success = self._prepare_repository()
            elif stage.name == "Clean Build Artifacts":
                success = self._clean_artifacts()
            elif stage.name == "Build Rust Core (/app)":
                success = self._build_rust_app()
            elif stage.name == "Run Tests":
                success = self._run_tests()
            elif stage.name == "Build Frontend":
                success = self._build_frontend()
            elif stage.name == "Build Tauri Application":
                success = self._build_tauri()
            elif stage.name == "Package Distribution":
                success = self._package_distribution()
            elif stage.name == "Finalization":
                success = self._finalize()
            else:
                self.log_queue.put(f"⚠ Unknown stage: {stage.name}")
                success = True  # Don't fail on unknown stages
            
        except Exception as e:
            error_msg = f"Stage failed with exception: {str(e)}"
            stage.error_message = error_msg
            self.log_queue.put(f"❌ {error_msg}")
            self._log_message(f"[ERROR] {error_msg}")
            success = False
        
        stage.end_time = time.time()
        if self.cancelled and not success and stage.error_message is None:
            stage.error_message = "Cancelled by user"
        stage.status = BuildStatus.SUCCESS if success else BuildStatus.FAILED
        
        return success
    
    def _create_header_panel(self) -> Panel:
        """Create animated header panel"""
        self.spinner_idx = (self.spinner_idx + 1) % len(self.spinner_frames)
        spinner = self.spinner_frames[self.spinner_idx]
        
        # Animated title with gradient effect (using Unicode box drawing)
        title_text = Text()
        title_text.append("╔═══════════════════════════════════════════════════════════╗\n", style="bold cyan")
        title_text.append("║ ", style="bold cyan")
        title_text.append(f" {SYMBOLS['rocket']} ", style="bold red")
        title_text.append(PROJECT_NAME.upper(), style="bold white")
        title_text.append(f" {SYMBOLS['rocket']} ", style="bold red")
        title_text.append(" ║\n", style="bold cyan")
        title_text.append("╚═══════════════════════════════════════════════════════════╝\n", style="bold cyan")
        title_text.append(f"    Version {VERSION} • Build System", style="dim white")
        
        if self.current_stage_idx >= 0:
            title_text.append(f" {spinner}", style="bold cyan")
        
        return Panel(
            Align.center(title_text),
            style="bold cyan on grey11",
            border_style="bold cyan",
            box=box.DOUBLE_EDGE
        )
    
    def _create_progress_panel(self) -> Panel:
        """Create progress panel with animated bar"""
        total_stages = len(self.stages)
        completed_stages = sum(1 for s in self.stages if s.status == BuildStatus.SUCCESS)
        failed_stages = sum(1 for s in self.stages if s.status == BuildStatus.FAILED)
        
        progress_percent = (completed_stages / total_stages * 100) if total_stages > 0 else 0
        
        # Create progress bar
        bar_width = 50
        filled = int(bar_width * progress_percent / 100)
        
        # Pulsing effect for active bar
        self.pulse_state = (self.pulse_state + 1) % 4
        pulse_chars = ['▁', '▃', '▅', '▇']
        pulse_char = pulse_chars[self.pulse_state]
        
        bar_text = Text()
        bar_text.append("█" * filled, style="bold green")
        if filled < bar_width and self.current_stage_idx >= 0 and self.current_stage_idx < total_stages:
            if self.stages[self.current_stage_idx].status == BuildStatus.RUNNING:
                bar_text.append(pulse_char, style="bold yellow")
                filled += 1
        bar_text.append("░" * (bar_width - filled), style="dim white")
        
        # Progress text
        progress_text = Text()
        progress_text.append(f"\n{SYMBOLS['arrow']} Progress: ", style="bold white")
        progress_text.append(f"{progress_percent:.1f}%", style="bold cyan")
        progress_text.append(f" ({completed_stages}/{total_stages} stages)\n\n", style="dim white")
        
        progress_text.append(bar_text)
        progress_text.append("\n\n")
        
        # Stats
        stats = Text()
        stats.append(f"{SYMBOLS['check']} Completed: ", style="bold green")
        stats.append(f"{completed_stages}  ", style="bold white")
        
        if failed_stages > 0:
            stats.append(f"{SYMBOLS['cross']} Failed: ", style="bold red")
            stats.append(f"{failed_stages}  ", style="bold white")
        
        if self.start_time:
            elapsed = time.time() - self.start_time
            stats.append(f"{SYMBOLS['hourglass']} Elapsed: ", style="bold yellow")
            
            if elapsed < 60:
                stats.append(f"{elapsed:.0f}s", style="bold white")
            elif elapsed < 3600:
                minutes = int(elapsed // 60)
                seconds = int(elapsed % 60)
                stats.append(f"{minutes}m {seconds}s", style="bold white")
            else:
                hours = int(elapsed // 3600)
                minutes = int((elapsed % 3600) // 60)
                stats.append(f"{hours}h {minutes}m", style="bold white")
        
        progress_text.append(stats)
        
        return Panel(
            progress_text,
            title=f"[bold cyan]{SYMBOLS['lightning']} Build Progress",
            border_style="bold cyan",
            box=box.ROUNDED
        )
    
    def _create_stages_panel(self) -> Panel:
        """Create stages table with status indicators"""
        table = Table(show_header=True, header_style="bold magenta", box=box.SIMPLE_HEAD)
        table.add_column("#", style="dim", width=3)
        table.add_column("Stage", style="cyan", min_width=30)
        table.add_column("Status", justify="center", width=10)
        table.add_column("Duration", justify="right", width=12)
        
        for idx, stage in enumerate(self.stages, 1):
            # Stage number
            num_style = "bold cyan" if idx - 1 == self.current_stage_idx else "dim"
            
            # Stage name with pulsing effect for active stage
            name_text = Text(stage.name)
            if idx - 1 == self.current_stage_idx and stage.status == BuildStatus.RUNNING:
                # Pulsing active stage
                name_text.stylize(f"bold cyan {'blink' if self.pulse_state % 2 == 0 else ''}")
            elif stage.status == BuildStatus.SUCCESS:
                name_text.stylize("green")
            elif stage.status == BuildStatus.FAILED:
                name_text.stylize("red")
            
            # Status with icon and color
            icon, color, text = stage.status.value
            status_text = Text(f"{icon} {text}", style=color)
            
            # Duration
            duration = stage.duration_str if stage.start_time else "-"
            duration_style = "bold white" if stage.status == BuildStatus.SUCCESS else "dim"
            
            table.add_row(
                str(idx),
                name_text,
                status_text,
                duration,
                style=num_style
            )
        
        return Panel(
            table,
            title=f"[bold magenta]{SYMBOLS['wrench']} Build Stages",
            border_style="bold magenta",
            box=box.ROUNDED
        )
    
    def _create_log_panel(self) -> Panel:
        """Create scrollable log panel"""
        # Process log queue
        try:
            while True:
                msg = self.log_queue.get_nowait()
                self.log_buffer.append(msg)
                if len(self.log_buffer) > self.max_log_lines:
                    self.log_buffer.pop(0)
        except Empty:
            pass
        
        # Create log text with syntax highlighting
        log_text = Text()
        for line in self.log_buffer[-20:]:  # Show last 20 lines
            if '[ERROR]' in line or '❌' in line or 'Failed' in line:
                log_text.append(line + "\n", style="bold red")
            elif '✓' in line or '✅' in line or 'Success' in line:
                log_text.append(line + "\n", style="bold green")
            elif '⚠' in line or 'Warning' in line:
                log_text.append(line + "\n", style="bold yellow")
            elif line.startswith('  '):
                log_text.append(line + "\n", style="dim white")
            else:
                log_text.append(line + "\n", style="white")
        
        return Panel(
            log_text,
            title=f"[bold yellow]{SYMBOLS['info']} Build Log (last 20 lines)",
            border_style="bold yellow",
            box=box.ROUNDED,
            height=12
        )
    
    def _create_system_panel(self) -> Panel:
        """Create system resource monitor panel"""
        cpu_percent = psutil.cpu_percent(interval=0.1)
        memory = psutil.virtual_memory()
        
        # CPU bar
        cpu_bar_width = 20
        cpu_filled = int(cpu_bar_width * cpu_percent / 100)
        cpu_color = "green" if cpu_percent < 50 else "yellow" if cpu_percent < 80 else "red"
        
        # Memory bar
        mem_bar_width = 20
        mem_filled = int(mem_bar_width * memory.percent / 100)
        mem_color = "green" if memory.percent < 50 else "yellow" if memory.percent < 80 else "red"
        
        system_text = Text()
        system_text.append(f"{SYMBOLS['lightning']} CPU: ", style="bold white")
        system_text.append(f"{cpu_percent:5.1f}% ", style=f"bold {cpu_color}")
        system_text.append("█" * cpu_filled, style=cpu_color)
        system_text.append("░" * (cpu_bar_width - cpu_filled), style="dim")
        system_text.append("\n")
        
        system_text.append(f"{SYMBOLS['package']} RAM: ", style="bold white")
        system_text.append(f"{memory.percent:5.1f}% ", style=f"bold {mem_color}")
        system_text.append("█" * mem_filled, style=mem_color)
        system_text.append("░" * (mem_bar_width - mem_filled), style="dim")
        
        return Panel(
            system_text,
            title=f"[bold green]{SYMBOLS['fire']} System Resources",
            border_style="bold green",
            box=box.ROUNDED
        )
    
    def _create_layout(self) -> Layout:
        """Create main layout"""
        layout = Layout()
        
        layout.split_column(
            Layout(name="header", size=7),
            Layout(name="main", ratio=1),
            Layout(name="footer", size=3)
        )
        
        layout["main"].split_row(
            Layout(name="left", ratio=2),
            Layout(name="right", ratio=1)
        )
        
        layout["left"].split_column(
            Layout(name="progress", size=12),
            Layout(name="stages", ratio=1),
        )
        
        layout["right"].split_column(
            Layout(name="log", ratio=2),
            Layout(name="system", size=6)
        )
        
        # Update panels
        layout["header"].update(self._create_header_panel())
        layout["progress"].update(self._create_progress_panel())
        layout["stages"].update(self._create_stages_panel())
        layout["log"].update(self._create_log_panel())
        layout["system"].update(self._create_system_panel())
        
        # Footer
        footer_status = Text()
        footer_status.append(f"{SYMBOLS['info']} ", style="bold cyan")
        
        if self.cancelled:
            footer_status.append("Build CANCELLED", style="bold red")
        elif self.paused:
            footer_status.append("Build PAUSED - type 'resume' to continue", style="bold yellow")
        elif self.current_stage_idx >= 0 and self.current_stage_idx < len(self.stages):
            current_stage = self.stages[self.current_stage_idx]
            footer_status.append(f"Current: {current_stage.description}", style="bold white")
        else:
            footer_status.append("Ready to build", style="bold green")
        
        footer_content = Text()
        footer_content.append_text(footer_status)
        if self.controls_enabled:
            footer_content.append("\n")
            footer_content.append("Controls: ", style="bold white")
            footer_content.append("[S]tart ", style="bold green")
            footer_content.append("[P]ause ", style="bold yellow")
            footer_content.append("[R]esume ", style="bold cyan")
            footer_content.append("[C]ancel ", style="bold red")
            footer_content.append("[E]xport report", style="bold magenta")
        
        layout["footer"].update(Panel(
            Align.center(footer_content),
            style="on grey11",
            border_style="dim"
        ))
        
        return layout
    
    def build(self) -> bool:
        """Execute the build process with live UI"""
        self.start_time = time.time()
        self.log_queue.put(f"🚀 Starting build of {PROJECT_NAME}")
        self.log_queue.put(f"📋 {len(self.stages)} stages planned\n")
        
        if self.controls_enabled:
            self.log_queue.put("⏳ Waiting for start command or auto-start in 1s...")
            self.start_event.wait()
            self.log_queue.put("▶️ Build started!\n")
        
        success = True
        
        with Live(self._create_layout(), refresh_per_second=4, console=self.console) as live:
            for idx, stage in enumerate(self.stages):
                if self.cancelled:
                    self.log_queue.put("\n🛑 Build cancelled by user")
                    success = False
                    break
                
                if not self.pause_event.is_set():
                    self.paused = True
                    while not self.pause_event.is_set() and not self.cancelled:
                        live.update(self._create_layout())
                        time.sleep(0.5)
                    self.paused = False
                    if self.cancelled:
                        self.log_queue.put("\n🛑 Build cancelled by user")
                        success = False
                        break
                
                self.current_stage_idx = idx
                live.update(self._create_layout())
                
                self.log_queue.put(f"\n{'='*60}")
                self.log_queue.put(f"🔨 Stage {idx+1}/{len(self.stages)}: {stage.name}")
                self.log_queue.put(f"{'='*60}\n")
                
                stage_success = self._execute_stage(stage)
                
                if not stage_success:
                    success = False
                    self.log_queue.put(f"\n❌ Build failed at stage: {stage.name}")
                    
                    # Update UI one more time to show failure
                    live.update(self._create_layout())
                    time.sleep(2)  # Give user time to see the error
                    break
                
                live.update(self._create_layout())
                time.sleep(0.3)  # Small delay for animation
            
            self.current_stage_idx = len(self.stages)
            self.end_time = time.time()
            
            # Final update
            live.update(self._create_layout())
            time.sleep(1)
        
        return success
    
    def generate_error_report(self) -> Path:
        """Generate detailed error report"""
        report_dir = self.config.project_dir / "build_reports"
        report_dir.mkdir(exist_ok=True)
        
        timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
        report_path = report_dir / f"error_report_{timestamp}.zip"
        
        with zipfile.ZipFile(report_path, 'w', zipfile.ZIP_DEFLATED) as zipf:
            # Add build log
            if self.config.log_file and self.config.log_file.exists():
                zipf.write(self.config.log_file, "build.log")
            
            # Add stage information
            stages_info = {
                'stages': [
                    {
                        'name': stage.name,
                        'description': stage.description,
                        'status': stage.status.name,
                        'duration': stage.duration,
                        'error': stage.error_message
                    }
                    for stage in self.stages
                ],
                'system': {
                    'platform': platform.system(),
                    'platform_version': platform.version(),
                    'architecture': platform.machine(),
                    'python_version': platform.python_version(),
                    'cpu_count': psutil.cpu_count(),
                    'memory_total': psutil.virtual_memory().total,
                }
            }
            
            zipf.writestr("stages.json", json.dumps(stages_info, indent=2))
            
            # Add system info
            system_info = {
                'platform': platform.system(),
                'release': platform.release(),
                'version': platform.version(),
                'machine': platform.machine(),
                'processor': platform.processor(),
                'python_version': sys.version,
            }
            
            zipf.writestr("system_info.json", json.dumps(system_info, indent=2))
        
        return report_path
    
    def print_summary(self, success: bool):
        """Print build summary"""
        self.console.print("\n" + "="*70 + "\n")
        
        if success:
            summary_text = Text()
            summary_text.append(f"{SYMBOLS['sparkles']} ", style="bold yellow")
            summary_text.append("BUILD SUCCESSFUL", style="bold green")
            summary_text.append(f" {SYMBOLS['sparkles']}", style="bold yellow")
            
            panel = Panel(
                Align.center(summary_text),
                style="bold green",
                border_style="bold green",
                box=box.DOUBLE_EDGE
            )
            self.console.print(panel)
            
            # Print output location
            self.console.print(f"\n📦 Build artifacts:", style="bold white")
            self.console.print(f"   {self.config.output_dir}", style="bold cyan")
            
            if self.config.log_file:
                self.console.print(f"\n📝 Build log:", style="bold white")
                self.console.print(f"   {self.config.log_file}", style="dim")
        else:
            summary_text = Text()
            summary_text.append(f"{SYMBOLS['cross']} ", style="bold red")
            summary_text.append("BUILD FAILED", style="bold red")
            summary_text.append(f" {SYMBOLS['cross']}", style="bold red")
            
            panel = Panel(
                Align.center(summary_text),
                style="bold red",
                border_style="bold red",
                box=box.DOUBLE_EDGE
            )
            self.console.print(panel)
            
            # Print failed stage
            failed_stages = [s for s in self.stages if s.status == BuildStatus.FAILED]
            if failed_stages:
                self.console.print(f"\n❌ Failed at stage:", style="bold red")
                for stage in failed_stages:
                    self.console.print(f"   • {stage.name}", style="red")
                    if stage.error_message:
                        self.console.print(f"     {stage.error_message}", style="dim red")
            
            # Offer error report
            self.console.print(f"\n📊 Generate error report? (y/N): ", style="bold yellow", end="")
            
            # Note: In automated environment, we'll just generate it
            self.console.print("(auto-generating for automation)")
            report_path = self.generate_error_report()
            self.console.print(f"   Error report saved: {report_path}", style="cyan")
        
        # Print timing
        if self.start_time and self.end_time:
            total_duration = self.end_time - self.start_time
            self.console.print(f"\n⏱️  Total build time: ", style="bold white", end="")
            
            if total_duration < 60:
                self.console.print(f"{total_duration:.1f}s", style="bold cyan")
            elif total_duration < 3600:
                minutes = int(total_duration // 60)
                seconds = int(total_duration % 60)
                self.console.print(f"{minutes}m {seconds}s", style="bold cyan")
            else:
                hours = int(total_duration // 3600)
                minutes = int((total_duration % 3600) // 60)
                self.console.print(f"{hours}h {minutes}m", style="bold cyan")
        
        self.console.print("\n" + "="*70 + "\n")
    
    def cleanup(self):
        """Cleanup resources"""
        if getattr(self, "auto_start_timer", None):
            try:
                self.auto_start_timer.cancel()
            except Exception:
                pass
        if getattr(self, 'log_file', None):
            self.log_file.close()


# CLI Interface
@click.command()
@click.option('--branch', default=DEFAULT_BRANCH, help='Git branch to build')
@click.option('--release', is_flag=True, help='Build in release mode (optimized)')
@click.option('--clean', is_flag=True, help='Clean build artifacts before building')
@click.option('--no-gui', is_flag=True, help='Skip GUI build (backend only)')
@click.option('--skip-tests', is_flag=True, help='Skip running tests')
@click.option('--output-dir', type=click.Path(), help='Output directory for build artifacts')
@click.option('--verbose', is_flag=True, help='Verbose output')
@click.version_option(VERSION)
def main(branch: str, release: bool, clean: bool, no_gui: bool, 
         skip_tests: bool, output_dir: Optional[str], verbose: bool):
    """
    CookieOne Cross-Platform Build Script
    
    A beautiful, animated build system for the CookieOne Voice Assistant project.
    
    Examples:
    
        # Standard debug build
        python build_cookieone.py
        
        # Release build with cleanup
        python build_cookieone.py --release --clean
        
        # Build only backend (no GUI)
        python build_cookieone.py --no-gui
        
        # Quick build without tests
        python build_cookieone.py --skip-tests
    """
    
    # Print banner
    console = Console()
    banner = Text()
    banner.append("╔═══════════════════════════════════════════════════════════╗\n", style="bold cyan")
    banner.append("║                                                           ║\n", style="bold cyan")
    banner.append("║  ", style="bold cyan")
    banner.append("🚀 CookieOne Build System", style="bold white")
    banner.append("                          ║\n", style="bold cyan")
    banner.append("║  ", style="bold cyan")
    banner.append(f"Version {VERSION}", style="dim white")
    banner.append("                                          ║\n", style="bold cyan")
    banner.append("║                                                           ║\n", style="bold cyan")
    banner.append("╚═══════════════════════════════════════════════════════════╝", style="bold cyan")
    
    console.print("\n")
    console.print(Align.center(banner))
    console.print("\n")
    
    # Create build configuration
    config = BuildConfig(
        branch=branch,
        release_mode=release,
        clean=clean,
        no_gui=no_gui,
        skip_tests=skip_tests,
        output_dir=Path(output_dir) if output_dir else None,
        verbose=verbose,
        project_dir=Path.cwd()
    )
    
    # Print configuration
    config_table = Table(show_header=False, box=box.ROUNDED, border_style="dim")
    config_table.add_column("Setting", style="bold cyan")
    config_table.add_column("Value", style="white")
    
    config_table.add_row("Branch", config.branch)
    config_table.add_row("Build Mode", "Release (optimized)" if config.release_mode else "Debug")
    config_table.add_row("Clean Build", "Yes" if config.clean else "No")
    config_table.add_row("GUI", "Disabled" if config.no_gui else "Enabled")
    config_table.add_row("Tests", "Skipped" if config.skip_tests else "Enabled")
    config_table.add_row("Output Directory", str(config.output_dir) if config.output_dir else "dist/")
    
    console.print(Panel(
        config_table,
        title="[bold cyan]⚙️  Build Configuration",
        border_style="cyan"
    ))
    console.print("\n")
    
    # Create and run build controller
    controller = BuildController(config)
    
    try:
        success = controller.build()
        controller.print_summary(success)
        controller.cleanup()
        
        sys.exit(0 if success else 1)
        
    except KeyboardInterrupt:
        console.print("\n\n⚠️  Build interrupted by user", style="bold yellow")
        controller.cancelled = True
        controller.cleanup()
        sys.exit(130)
    except Exception as e:
        console.print(f"\n\n❌ Unexpected error: {e}", style="bold red")
        import traceback
        traceback.print_exc()
        controller.cleanup()
        sys.exit(1)


if __name__ == "__main__":
    main()
