# Voraussetzungen  

Zur Nutzung des Web-Scrapers wird Python (3.10 ^) benötigt.

1. Installiere Python 
    - Linux (Debian/Ubuntu)
    ```bash
    sudo apt update
    sudo apt install python3 python3-pip -y
    python3 --version
    pip3 --version 
    ```
    - Windows
        - Besuche die offizielle Website: https://www.python.org/downloads/windows/
        - Lade den Installer für Windows x64 herunter (z. B. Python 3.11.6)
        - Starte den Installer:
        - ✅ Haken setzen bei "Add Python to PATH"
        - Dann auf Install Now klicken
        - Nach der Installation: Öffne die Eingabeaufforderung (CMD) und überprüfe Installation:
        ```
        python --version
        pip --version
        ```

# Google Chrome Binary installieren 
- Überprüfe, ob Google Chrome installiert ist
    - Linux
    ```
    which google-chrome
    ```
    - Windows CMD
    ```
    where chrome # oder
    Get-Command chrome
    ```
- Falls Binary nicht existiert:
    -  Linux (Ubuntu/Debian)
    ```
    wget https://dl.google.com/linux/direct/google-chrome-stable_current_amd64.deb
    sudo apt update
    sudo apt install ./google-chrome-stable_current_amd64.deb -y
    google-chrome --version
    ```
    - Windows 
        - Gehe zu https://www.google.com/chrome/
        - Klicke auf "Chrome herunterladen" und führe die .exe Datei aus
    
# Ausführung des Web-Scrapers

- Gehe auf den Ordner web_scraping

- Lege eine .env Datei an mit deinem Username und Passwort von der THA:
```
WEB_USERNAME="<dein Username>"
WEB_PW="<dein Passwort>"
```
- Bei der ersten Ausführung den Befehl `make ` ausführen, damit die notwendigen python libraries installiert werden können.
- Für alle nächsten Ausführungen: `make calculate` 