# Präsentationsschicht (Anwendungsseite)

Die Präsentationsschicht ist die Schnittstelle, mit der der Benutzer direkt interagiert, und umfasst verschiedene Anwendungen und Frontend-Oberflächen. Diese Ebene befindet sich an der Spitze der dreischichtigen Architektur und bietet Dienste, indem sie die Kernmodule der Implementationsschicht aufruft.

## Gesamtarchitektur

Die Präsentationsschicht besteht aus zwei Hauptteilen:
- **Frontend-Anwendungen** – befindet sich im Verzeichnis `frontends/` und bietet die Benutzeroberfläche
- **Backend-Anwendungen** – befindet sich im Verzeichnis `backends/` und bietet serverseitige Unterstützung

### Komponentenbeziehungsdiagramm der Präsentationsschicht

```
┌─────────────────────────────────────────────────────────────┐
│                    Frontend-Anwendungen (frontends/)         │
├─────────────────────────────────────────────────────────────┤
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐    │
│  │  ai-company  │  │  ai-empire   │  │  ai-planet   │    │
│  │  (Hauptapp)  │  │  (Empire-Mod) │  │  (Planet-Mod) │    │
│  └──────────────┘  └──────────────┘  └──────────────┘    │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐    │
│  │  ai-waifu    │  │  client-h5    │  │ client-desktop│  │
│  │  (Charakter)  │  │  (H5-Client)  │  │  (Desktop-Client)│ │
│  └──────────────┘  └──────────────┘  └──────────────┘    │
│  ┌──────────────┐  ┌──────────────┐                        │
│  │client-mobile │  │ client-shared │                        │
│  │  (Mobile)    │  │  (Shared Lib) │                        │
│  └──────────────┘  └──────────────┘                        │
└─────────────────────────────────────────────────────────────┘
                            ↓ HTTP/WebSocket
┌─────────────────────────────────────────────────────────────┐
│                    Backend-Anwendungen (backends/)           │
├─────────────────────────────────────────────────────────────┤
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐    │
│  │  ai-company  │  │  ai-empire   │  │  ai-planet   │    │
│  │  (Hauptapp)  │  │  (Empire-Mod) │  │  (Planet-Mod) │    │
│  └──────────────┘  └──────────────┘  └──────────────┘    │
│  ┌──────────────┐                                            │
│  │  ai-waifu    │                                            │
│  │  (Charakter)  │                                            │
│  └──────────────┘                                            │
└─────────────────────────────────────────────────────────────┘
                            ↓ Aufruf
┌─────────────────────────────────────────────────────────────┐
│           Implementationsschicht (augur-* Kernmodule)        │
└─────────────────────────────────────────────────────────────┘
```

## Detailierte Beschreibung der Frontend-Anwendungen

### 1. ai-company (Hauptanwendung)

**Verzeichnisposition**: `frontends/ai-company/`

**Technologie-Stack**:
- Vue 3 + TypeScript
- Vite-Build-Tool
- Element Plus UI-Komponentenbibliothek
- Vue Router Routing
- Pinia Zustandsverwaltung
- UnoCSS atomare CSS
- Fluent Vue Internationalisierung

**Port**: Der Standardport des Entwicklungsservers wird über die Vite-Konfiguration festgelegt

**Funktionalitäten und Verantwortlichkeiten**:
- Marketing-Startseite
- Benutzeranmeldung/Registrierung
- Unternehmensverwaltung
- Mitarbeiterverwaltung
- Teamverwaltung
- Projektverwaltung
- Forumfunktion
- Administratorfunktionen

**Hauptansichten**:
- `Home.vue` – Startseite
- `Login.vue` – Anmeldeseite
- `Register.vue` – Registrierungsseite
- `Company.vue` – Unternehmensliste
- `CompanyDetail.vue` – Unternehmensdetails
- `Employees.vue` – Mitarbeiterliste
- `EmployeeDetail.vue` – Mitarbeiterdetails
- `Team.vue` – Teamliste
- `TeamDetail.vue` – Teamdetails
- `Project.vue` – Projektliste
- `ProjectDetail.vue` – Projektdetails
- `Forum.vue` – Forum
- `ForumDetail.vue` – Forumdetails
- `Dashboard.vue` – Dashboard
- `Download.vue` – Downloadseite
- `admin/AdminDashboard.vue` – Administrator-Dashboard
- `admin/Users.vue` – Benutzermanagement

### 2. ai-empire (Empire-Modul)

**Verzeichnisposition**: `frontends/ai-empire/`

**Technologie-Stack**:
- Vue 3 + TypeScript
- Vite + Vite-SSG
- Element Plus UI-Komponentenbibliothek
- Vue Router Routing
- Pinia Zustandsverwaltung
- UnoCSS atomare CSS
- Fluent Vue Internationalisierung
- TipTap Rich-Text-Editor

**Funktionalitäten und Verantwortlichkeiten**:
- Empire-Verwaltungsoberfläche
- AI-Berater (Advisor) Verwaltung
- AI-Berater (Consultant) Verwaltung
- Legion Verwaltung
- Great Work Verwaltung
- Benutzerauthentifizierung
- Dashboard-Anzeige

**Hauptansichten**:
- `Home.vue` – Startseite
- `Login.vue` – Anmeldeseite
- `Register.vue` – Registrierungsseite
- `Empire.vue` – Empire-Liste
- `EmpireDetail.vue` – Empire-Details
- `Advisors.vue` – Beraterliste
- `AdvisorDetail.vue` – Beraterdetails
- `Consultants.vue` – Beraterliste
- `ConsultantDetail.vue` – Beraterdetails
- `Legion.vue` – Legion-Liste
- `LegionDetail.vue` – Legion-Details
- `GreatWork.vue` – Great Work-Liste
- `GreatWorkDetail.vue` – Great Work-Details

**Besonderheiten**:
- AI-Mitarbeiterverwaltung: AI-Agenten als Unternehmensmitarbeiter
- Rollenbasierte AI-Hilfen: Kundenservice, Finanzen, Marketing, Verwaltungsassistent, Recht usw.
- Wissensmanagementsystem
- Rich-Text-Editor-Funktion

### 3. ai-planet (Planet-Modul)

**Verzeichnisposition**: `frontends/ai-planet/`

**Technologie-Stack**:
- Vue 3 + TypeScript
- Vite + Vite-SSG
- Element Plus UI-Komponentenbibliothek
- Vue Router Routing
- Pinia Zustandsverwaltung
- UnoCSS atomare CSS
- Fluent Vue Internationalisierung
- TipTap Rich-Text-Editor

**Funktionalitäten und Verantwortlichkeiten**:
- Planet-bezogene Funktionsoberflächen
- Benutzerauthentifizierung

**Hauptansichten**:
- `Home.vue` – Startseite
- `Auth.vue` – Authentifizierungsseite

### 4. ai-waifu (Charakter-Modul)

**Verzeichnisposition**: `frontends/ai-waifu/`

**Technologie-Stack**:
- Vue 3 + TypeScript
- Vite + Vite-SSG
- Element Plus UI-Komponentenbibliothek
- Vue Router Routing
- Pinia Zustandsverwaltung
- UnoCSS atomare CSS
- Fluent Vue Internationalisierung
- TipTap Rich-Text-Editor

**Funktionalitäten und Verantwortlichkeiten**:
- AI-Charakter-Chat-Oberfläche
- Charakterinteraktion

**Hauptansichten**:
- `Home.vue` – Startseite
- `Chat.vue` – Chat-Oberfläche

### 5. client-h5 (H5-Client)

**Verzeichnisposition**: `frontends/client-h5/`

**Technologie-Stack**:
- Vue 3 + TypeScript
- Vite-Build-Tool
- Element Plus UI-Komponentenbibliothek
- Vue Router Routing
- Pinia Zustandsverwaltung
- UnoCSS atomare CSS

**Funktionalitäten und Verantwortlichkeiten**:
- Mobile H5-Anwendung
- Arbeitsplatzansicht
- Augur-Kollaborationsmodus
- WeChat Work-Stilmodus
- Anwendungsverwaltung
- Nachrichtenzentrale
- Kontaktverwaltung
- Kalender
- Besprechungen
- E-Mail
- Wissensdatenbank
- Gedächtnisverwaltung
- Projektmanagement

**Besonderheiten**:
- Arbeitsmoduswechsel: Augur-Kollaborationsmodus / WeChat Work-Stilmodus
- Multi-Anwendungs-Integration: Anwesenheit, CRM, Vertrag, Finanzen, HR, Leistung, Einstellung, Wiki, Workflow usw.
- Mehrere Layout-Modi

### 6. client-desktop (Desktop-Client)

**Verzeichnisposition**: `frontends/client-desktop/`

**Technologie-Stack**:
- Tauri (Rust + WebView)
- Plattformübergreifende Desktop-Anwendung

**Funktionalitäten und Verantwortlichkeiten**:
- Desktop-Anwendung
- Native Desktop-Erfahrung

### 7. client-mobile (Mobile-Client)

**Verzeichnisposition**: `frontends/client-mobile/`

**Technologie-Stack**:
- Tauri (Rust + WebView)
- Mobile Anwendung

**Funktionalitäten und Verantwortlichkeiten**:
- Mobile Anwendung

### 8. client-shared (Gemeinsame Bibliothek)

**Verzeichnisposition**: `frontends/client-shared/`

**Technologie-Stack**:
- Vue 3 + TypeScript

**Funktionalitäten und Verantwortlichkeiten**:
- Gemeinsame Komponentenbibliothek
- Gemeinsame Dienste (API-Aufrufe)
- Gemeinsame Zustandsverwaltung (Pinia-Stores)
- Gemeinsame Typdefinitionen
- Gemeinsame Hilfsfunktionen
- Gemeinsame Internationalisierungsressourcen
- Gemeinsame Plugins

**Enthaltene Inhalte**:
- `components/` – Gemeinsame Vue-Komponenten
- `services/` – API-Dienste und Mock-Daten
- `stores/` – Pinia-Zustandsverwaltung
- `types/` – TypeScript-Typdefinitionen
- `utils/` – Hilfsfunktionen
- `locales/` – Internationalisierungsressourcen
- `plugins/` – Vue-Plugins

## Detailierte Beschreibung der Backend-Anwendungen

### 1. ai-company (Hauptanwendungs-Backend)

**Verzeichnisposition**: `backends/ai-company/`

**Technologie-Stack**:
- Rust + Tokio asynchrone Laufzeit
- Axum Web-Framework
- Serde Serialisierung
- Tracing Logging
- rust-embed Einbettung statischer Ressourcen

**Port**: 4002

**Funktionalitäten und Verantwortlichkeiten**:
- Bereitstellen von HTTP-Diensten
- Einbetten und Bereitstellen von Frontend-Standardressourcen
- Routing und statische Dateidienste
- Serverseitige Unterstützung für das Hauptanwendungs-Frontend

**Hauptfunktionen**:
- Statische Dateidienste: Einbettung der Frontend-Build-Artefakte aus dem Verzeichnis `frontends/ai-company/dist`
- Single-Page-Anwendung Routing-Unterstützung: Alle nicht übereinstimmenden Pfade geben index.html zurück
- Automatische Erkennung von MIME-Typen für statische Ressourcen

### 2. ai-empire (Empire-Modul-Backend)

**Verzeichnisposition**: `backends/ai-empire/`

**Technologie-Stack**:
- Rust + Tokio asynchrone Laufzeit
- Axum Web-Framework
- Serde Serialisierung
- Tracing Logging
- rust-embed Einbettung statischer Ressourcen

**Funktionalitäten und Verantwortlichkeiten**:
- Bereitstellen von HTTP-Diensten
- Einbetten und Bereitstellen von Empire-Modul-Frontend-Standardressourcen
- Serverseitige Unterstützung für das Empire-Modul-Frontend

### 3. ai-planet (Planet-Modul-Backend)

**Verzeichnisposition**: `backends/ai-planet/`

**Technologie-Stack**:
- Rust + Tokio asynchrone Laufzeit
- Axum Web-Framework
- Serde Serialisierung
- Tracing Logging
- rust-embed Einbettung statischer Ressourcen

**Funktionalitäten und Verantwortlichkeiten**:
- Bereitstellen von HTTP-Diensten
- Einbetten und Bereitstellen von Planet-Modul-Frontend-Standardressourcen
- Serverseitige Unterstützung für das Planet-Modul-Frontend

### 4. ai-waifu (Charakter-Modul-Backend)

**Verzeichnisposition**: `backends/ai-waifu/`

**Technologie-Stack**:
- Rust + Tokio asynchrone Laufzeit
- Axum Web-Framework
- Serde Serialisierung
- Tracing Logging
- rust-embed Einbettung statischer Ressourcen

**Funktionalitäten und Verantwortlichkeiten**:
- Bereitstellen von HTTP-Diensten
- Einbetten und Bereitstellen von Charakter-Modul-Frontend-Standardressourcen
- Serverseitige Unterstützung für das Charakter-Modul-Frontend

## Beziehungen zwischen Anwendungen

### Beziehungen zwischen Frontend-Anwendungen

```
client-shared (Gemeinsame Bibliothek)
    ↑ Abhängigkeit
    ├─→ ai-company (Hauptanwendung)
    ├─→ ai-empire (Empire-Modul)
    ├─→ ai-planet (Planet-Modul)
    ├─→ ai-waifu (Charakter-Modul)
    └─→ client-h5 (H5-Client)

client-h5 (H5-Client)
    ↑ Referenz/Inspiration
    ├─→ client-desktop (Desktop-Client)
    └─→ client-mobile (Mobile-Client)
```

### Frontend-Backend-Paarungen

| Frontend-Anwendung | Backend-Anwendung | Beschreibung |
|-------------------|------------------|--------------|
| `ai-company` | `ai-company` | Hauptanwendung Frontend-Backend-Paarung |
| `ai-empire` | `ai-empire` | Empire-Modul Frontend-Backend-Paarung |
| `ai-planet` | `ai-planet` | Planet-Modul Frontend-Backend-Paarung |
| `ai-waifu` | `ai-waifu` | Charakter-Modul Frontend-Backend-Paarung |
| `client-h5` | (TBD) | H5-Client-Backend |
| `client-desktop` | (TBD) | Desktop-Client-Backend |
| `client-mobile` | (TBD) | Mobile-Client-Backend |

### Beziehung zur Implementationsschicht

Alle Backend-Anwendungen rufen letztendlich die `augur-*` Kernmodule der Implementationsschicht auf, um die Geschäftslogik auszuführen:

```
Präsentationsschicht (Backend-Anwendungen)
    ↓ Aufruf
Implementationsschicht (augur-* Kernmodule)
    ↓ Nutzung
Protokollschicht (Skynet-Protokoll)
```

## Bereitstellungsarchitektur

### Entwicklungsumgebung

- Frontend-Anwendungen laufen unabhängig über den Vite-Entwicklungsserver
- Backend-Anwendungen laufen unabhängig über Cargo
- Frontend und Backend kommunizieren über API-Aufrufe

### Produktionsumgebung

- Frontend-Build-Artefakte sind in die entsprechenden Backend-Anwendungen eingebettet (über rust-embed)
- Jede Backend-Anwendung läuft als unabhängiger Dienst und bietet vollständige Frontend- und Backend-Funktionalität
- Jeder Dienst hört auf einem anderen Port

## Technologieauswahl

### Frontend-Technologieauswahl

- **Vue 3**: Progressives JavaScript-Framework, bietet ausgezeichnete Entwicklererfahrung und Leistung
- **TypeScript**: Bietet Typsicherheit und verbessert die Code-Wartbarkeit
- **Vite**: Nächste Generation Frontend-Build-Tool, bietet extrem schnelle Entwicklererfahrung
- **Element Plus**: Auf Vue 3 basierende Komponentenbibliothek, bietet reichhaltige UI-Komponenten
- **Pinia**: Von Vue 3 offiziell empfohlene Zustandsverwaltungsbibliothek
- **UnoCSS**: Atomare CSS-Engine, bietet flexible Styling-Lösung
- **Fluent Vue**: Internationalisierungs-Lösung, unterstützt mehrere Sprachen

### Backend-Technologieauswahl

- **Rust**: Hochleistungsfähige, speichersichere Systemprogrammiersprache
- **Tokio**: Rust asynchrone Laufzeit, bietet hochleistungsfähige E/A-Verarbeitung
- **Axum**: Ergonomisches und modulares Web-Framework, bietet gute Entwicklererfahrung
- **rust-embed**: Einbettung von Frontend-Build-Artefakten in Rust-Binärdateien, vereinfacht die Bereitstellung

## Entwicklungsleitfaden

### Frontend-Entwicklung

```bash
# In das Frontend-Anwendungsverzeichnis wechseln
cd frontends/ai-company

# Abhängigkeiten installieren
npm install

# Entwicklungsserver starten
npm run dev

# Produktionsversion bauen
npm run build
```

### Backend-Entwicklung

```bash
# In das Backend-Anwendungsverzeichnis wechseln
cd backends/ai-company

# Entwicklungsserver ausführen
cargo run

# Produktionsversion bauen
cargo build --release
```

### Vollständiger Entwicklungsablauf

1. Zuerst die Frontend-Anwendung bauen
2. Dann die Backend-Anwendung ausführen (die Frontend-Build-Artefakte werden eingebettet)

```bash
# Frontend bauen
cd frontends/ai-company
npm run build

# Backend ausführen
cd ../../backends/ai-company
cargo run
```
