import json
import os
from collections import defaultdict
from pathlib import Path

root = Path(__file__).resolve().parents[1]
commands_path = root / "app" / "commands" / "commands.json"
markdown_path = root / "docs" / "command_catalog.md"
markdown_path.parent.mkdir(parents=True, exist_ok=True)

commands = []
id_counters = defaultdict(int)
markdown_sections = defaultdict(list)


def add_command(prefix: str, category: str, name: str, description: str, keywords, action, icon=None):
    id_counters[prefix] += 1
    command_id = f"{prefix}_{id_counters[prefix]:03d}"
    entry = {
        "id": command_id,
        "name": name,
        "description": description,
        "icon": icon,
        "keywords": [k.lower() for k in keywords],
        "action": action,
    }
    commands.append(entry)
    markdown_sections[category].append((command_id, name, description, action))


# -------------------------
# Category: Applications
# -------------------------

browser_apps = [
    {
        "name": "Google Chrome",
        "exe": r"C:\\Program Files\\Google\\Chrome\\Application\\chrome.exe",
        "aliases": ["chrome", "хром", "google chrome"],
    },
    {
        "name": "Mozilla Firefox",
        "exe": r"C:\\Program Files\\Mozilla Firefox\\firefox.exe",
        "aliases": ["firefox", "файрфокс", "mozilla"],
    },
    {
        "name": "Microsoft Edge",
        "exe": r"C:\\Program Files (x86)\\Microsoft\\Edge\\Application\\msedge.exe",
        "aliases": ["edge", "эдж", "microsoft edge"],
    },
    {
        "name": "Opera",
        "exe": r"C:\\Users\\%USERNAME%\\AppData\\Local\\Programs\\Opera\\opera.exe",
        "aliases": ["opera", "опера", "браузер opera"],
    },
    {
        "name": "Brave",
        "exe": r"C:\\Program Files\\BraveSoftware\\Brave-Browser\\Application\\brave.exe",
        "aliases": ["brave", "брейв", "браузер brave"],
    },
    {
        "name": "Vivaldi",
        "exe": r"C:\\Users\\%USERNAME%\\AppData\\Local\\Vivaldi\\Application\\vivaldi.exe",
        "aliases": ["vivaldi", "вивальди", "браузер vivaldi"],
    },
    {
        "name": "Yandex Browser",
        "exe": r"C:\\Users\\%USERNAME%\\AppData\\Local\\Yandex\\YandexBrowser\\Application\\browser.exe",
        "aliases": ["yandex", "яндекс браузер", "yandex browser"],
    },
    {
        "name": "Tor Browser",
        "exe": r"C:\\Program Files\\Tor Browser\\Browser\\firefox.exe",
        "aliases": ["tor", "тор", "tor browser"],
    },
]

for browser in browser_apps:
    name = browser["name"]
    exe = browser["exe"]
    alias = browser["aliases"][0]
    readable_alias = browser["aliases"][1]

    # 1. Standard launch
    add_command(
        "app",
        "Приложения",
        f"Запуск {name}",
        f"Открывает браузер {name}.",
        [
            f"открой {alias}",
            f"запусти {alias}",
            name.lower(),
            f"браузер {readable_alias}",
        ],
        {
            "type": "run_process",
            "command": "cmd",
            "args": ["/C", f'"{exe}"'],
            "working_dir": None,
        },
    )

    # 2. Incognito / private mode
    add_command(
        "app",
        "Приложения",
        f"{name} в режиме инкогнито",
        f"Запускает {name} в приватном режиме.",
        [
            f"{alias} инкогнито",
            f"приватный режим {alias}",
            f"открой {alias} приватно",
            f"{readable_alias} без истории",
        ],
        {
            "type": "run_process",
            "command": "cmd",
            "args": ["/C", f'"{exe}" --incognito'],
            "working_dir": None,
        },
    )

    # 3. Open Gmail in browser
    add_command(
        "app",
        "Приложения",
        f"{name}: Gmail",
        f"Открывает Gmail в {name}.",
        [
            f"почта в {alias}",
            f"gmail через {alias}",
            f"открой gmail в {readable_alias}",
            "перейди в gmail",
        ],
        {
            "type": "run_process",
            "command": "cmd",
            "args": ["/C", f'"{exe}" https://mail.google.com'],
            "working_dir": None,
        },
    )

    # 4. Developer Tools mode
    add_command(
        "app",
        "Приложения",
        f"{name} для разработки",
        f"Запускает {name} с включенными инструментами разработчика.",
        [
            f"{alias} разработчик",
            f"режим разработчика {alias}",
            f"frontend {alias}",
            f"{readable_alias} для тестов",
        ],
        {
            "type": "run_process",
            "command": "cmd",
            "args": [
                "/C",
                f'"{exe}" --auto-open-devtools-for-tabs https://example.com',
            ],
            "working_dir": None,
        },
    )

# Later categories will be added here.

# After populating all categories, validate and output
if len(commands) != 800:
    raise SystemExit(f"Expected 800 commands, but generated {len(commands)}")

commands_json = {
    "commands": commands,
}

commands_path.parent.mkdir(parents=True, exist_ok=True)
with open(commands_path, "w", encoding="utf-8") as fp:
    json.dump(commands_json, fp, ensure_ascii=False, indent=2)

# Build markdown output
lines = ["# 📚 Полный каталог команд Cookie", ""]
for category in sorted(markdown_sections):
    entries = markdown_sections[category]
    lines.append(f"## {category}")
    lines.append("")
    lines.append("| ID | Название | Описание | Действие |")
    lines.append("| --- | --- | --- | --- |")
    for command_id, title, desc, action in entries:
        if action["type"] == "run_process":
            action_desc = f"{action['command']} {' '.join(action.get('args', []))}".strip()
        elif action["type"] == "respond_text":
            action_desc = action["text"]
        else:
            action_desc = action.get("file", "")
        action_desc = action_desc.replace("|", "\\|")
        lines.append(f"| `{command_id}` | {title} | {desc} | {action_desc} |")
    lines.append("")

with open(markdown_path, "w", encoding="utf-8") as fp:
    fp.write("\n".join(lines))

print(f"Сохранено {len(commands)} команд в {commands_path}")
print(f"Документация создана: {markdown_path}")
