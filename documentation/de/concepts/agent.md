# Mitarbeiter (Employee): Agent

Ein Mitarbeiter (Employee) ist ein Agent, der aus einer **Sammlung von Fähigkeiten** besteht und definiert, was ein Agent tun kann, was er gut kann und wie er mit anderen Agenten zusammenarbeitet.

## Konzeptdefinition

Ein **Mitarbeiter** ist ein Container und eine Kombination von Fähigkeiten, die durch die Kombination verschiedener Fähigkeiten eine vollständige Arbeitsfähigkeit bilden. Der Mitarbeiter selbst ist keine Fähigkeit, sondern eine Sammlung von Fähigkeiten.

## Kernelemente

### Grundinformationen

- **Mitarbeitername**: Der identifizierende Name des Agenten
- **Mitarbeiteravatar**: Visuelles Identifikationsmerkmal
- **Position**: Die Rollenbestimmung des Agenten
- **Beschreibung**: Detaillierte Fähigkeitseinleitung
- **Rollentyp**: Die Rollenklassifizierung des Agenten

### Fähigkeitssammlung

Der Mitarbeiter bildet Arbeitsfähigkeit durch die Kombination mehrerer Fähigkeiten:

- **Kernfähigkeiten**: Grundfähigkeiten (Kommunikation, Argumentation, Lernen, Gedächtnis usw.)
- **Fachfähigkeiten**: Fachfähigkeiten in bestimmten Bereichen (Programmierung, Analyse, Design usw.)
- **Werkzeugfähigkeiten**: Fähigkeit, bestimmte Werkzeuge zu verwenden (Entwicklungswerkzeuge, Designwerkzeuge, Bürowerkzeuge usw.)
- **Soft Skills**: Fähigkeiten zur zwischenmenschlichen Kommunikation und Teamarbeit

Weitere Informationen zum detaillierten Fähigkeitssystem finden Sie in [skills.md](skills.md).

### Fähigkeitskonfiguration

Der Mitarbeiter kann verschiedene Fähigkeiten konfigurieren, um seine Funktionen zu erweitern:

- **API-Integrationsfähigkeit**: Fähigkeit, externe APIs aufzurufen
- **MCP-Servicefähigkeit**: Fähigkeit, MCP (Model Context Protocol)-Dienste zu verwenden
- **Dateibearbeitungsfähigkeit**: Fähigkeit, Dateien zu lesen und zu schreiben
- **Netzwerkzugriffsfähigkeit**: Fähigkeit, auf Netzwerkressourcen zuzugreifen

Weitere Informationen zum detaillierten Fähigkeitssystem finden Sie in [capabilities.md](capabilities.md).

### Arbeitsstil

- **Reaktionsgeschwindigkeit**: Schnelle Reaktion / Gründliche Überlegung
- **Entscheidungsweise**: Entschlossene Entscheidungen / Gründliche Beratung
- **Kommunikationsstil**: Präzise und direkt / Detaillierte Erklärung
- **Risikopräferenz**: Risikobereit / Vorsichtig und konservativ

## Rolle des Mitarbeiters

### 1. Fähigkeitskombination

- Kombinieren mehrerer Fähigkeiten zu einer vollständigen Arbeitsfähigkeit
- Fähigkeiten arbeiten zusammen und bilden Synergien
- Flexibel auf verschiedene Aufgaben-Szenarien reagieren

### 2. Fähigkeitsstandardisierung

- Klar definieren der Fähigkeitsgrenzen des Agenten
- Einfacher für den Benutzer, den passenden Agenten auszuwählen
- Können verschiedene Agenten bewertet und verglichen werden

### 3. Stetige Weiterentwicklung

- Agenten können neue Fähigkeiten lernen
- Fähigkeiten können ständig verbessert und optimiert werden
- Können neue Fähigkeitserweiterungen hinzugefügt werden
- An neue Anforderungen und Szenarien anpassen

## Beispiel: Full-Stack-Entwickler-Agent

```
Full-Stack-Entwickler (Alex)
├── Grundinformationen
│   ├── Name: Alex
│   ├── Position: Senior Full-Stack-Entwicklungsingenieur
│   └── Kurzbeschreibung: 5 Jahre Entwicklungs Erfahrung, spezialisiert auf Web-Anwendungsentwicklung
├── Fähigkeitssammlung
│   ├── Kernfähigkeiten
│   │   ├── Kommunikationsfähigkeit: Fortgeschritten
│   │   ├── Argumentationsfähigkeit: Experte
│   │   ├── Lernfähigkeit: Fortgeschritten
│   │   └── Gedächtnisfähigkeit: Fortgeschritten
│   ├── Fachfähigkeiten
│   │   ├── Frontend-Entwicklung (React/Vue/TypeScript): Experte
│   │   ├── Backend-Entwicklung (Node.js/Python): Fortgeschritten
│   │   ├── Datenbankdesign: Experte
│   │   └── API-Design: Fortgeschritten
│   ├── Werkzeugfähigkeiten
│   │   ├── Git: Experte
│   │   ├── VS Code: Experte
│   │   ├── Docker: Fortgeschritten
│   │   └── CI/CD: Fortgeschritten
│   └── Soft Skills
│       ├── Teamarbeit: Fortgeschritten
│       ├── Codeprüfung: Experte
│       └── Technische Dokumentation: Fortgeschritten
├── Fähigkeitskonfiguration
│   ├── API-Integrationsfähigkeit: Aktiviert
│   ├── MCP-Servicefähigkeit: Aktiviert
│   ├── Dateibearbeitungsfähigkeit: Aktiviert
│   └── Netzwerkzugriffsfähigkeit: Aktiviert
└── Arbeitsstil
    ├── Reaktionsgeschwindigkeit: Schnelle Reaktion
    ├── Entscheidungsweise: Entschlossene Entscheidungen
    ├── Kommunikationsstil: Detaillierte Erklärung
    └── Codestil: Auf Wartbarkeit achten
```
