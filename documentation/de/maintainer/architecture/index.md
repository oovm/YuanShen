# Architekturdesign

Dieses Verzeichnis enthält Dokumente zu den Architekturdesignkonzepten und -prinzipien von AI Company.

## Dokumentationsliste

- [decentralization.md](decentralization.md) - Dezentrales Design: Kernsicherheitsdesignprinzipien des Systems
- [security-model.md](security-model.md) - Sicherheitsmodell: Architekturebenen-Sicherheitsimplementierungsanleitung basierend auf dem Skynet-Protokoll
- [presentation-layer.md](presentation-layer.md) - Präsentationsschicht: Detaillierte Beschreibung von Frontend-Anwendungen und Backend-Diensten
- [implementation-layer.md](implementation-layer.md) - Implementierungsschicht: Detaillierte Beschreibung der augur-* Kernmodule
- [protocol-layer.md](protocol-layer.md) - Protokollebene: Detaillierte Beschreibung der Skynet-Protokollmodule

## Drei-Schichten-Architektur

AI Company nutzt ein klares Drei-Schichten-Architekturdesign, wobei jede Schicht klare Verantwortlichkeiten und Grenzen hat.

### Architekturebenen-Beziehungsdiagramm

```
┌─────────────────────────────────────────────────────────────┐
│              Präsentationsschicht (Anwendungsseite)         │
├─────────────────────────────────────────────────────────────┤
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐    │
│  │  ai-company  │  │  ai-empire   │  │  ai-planet   │    │
│  │ (Hauptapp)   │  │ (Empire-Modul)│  │ (Planet-Modul)│   │
│  └──────────────┘  └──────────────┘  └──────────────┘    │
│  ┌──────────────┐  ┌──────────────┐                         │
│  │  ai-waifu    │  │Frontend-App  │                         │
│  │ (Charakter-  │  │ (Vue.js)      │                         │
│  │  Modul)      │  │               │                         │
│  └──────────────┘  └──────────────┘                         │
└─────────────────────────────────────────────────────────────┘
                            ↓ Aufruf
┌─────────────────────────────────────────────────────────────┐
│         Implementierungsschicht (augur-* Kernmodule)        │
├─────────────────────────────────────────────────────────────┤
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐    │
│  │augur-agent   │  │augur-orchestr│  │augur-organiz │    │
│  │(Agentenver-  │  │ator(Orchest- │  │ation(Organi- │    │
│  │ waltung)     │  │ rierung)      │  │ sation)      │    │
│  └──────────────┘  └──────────────┘  └──────────────┘    │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐    │
│  │augur-skill   │  │augur-memory  │  │augur-persist │    │
│  │(Fähigkeits-  │  │(Speichersys- │  │ence(Persis-  │    │
│  │ verwaltung)  │  │ tem)          │  │ tenz)        │    │
│  └──────────────┘  └──────────────┘  └──────────────┘    │
│  ┌──────────────┐  ┌──────────────┐                         │
│  │augur-file-sys│  │augur-types   │                         │
│  │tem(Dateisys- │  │(Typdefinitio-│                         │
│  │ tem)         │  │ nen)          │                         │
│  └──────────────┘  └──────────────┘                         │
└─────────────────────────────────────────────────────────────┘
                            ↓ Nutzung
┌─────────────────────────────────────────────────────────────┐
│               Protokollebene (Skynet-Protokoll)             │
├─────────────────────────────────────────────────────────────┤
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐    │
│  │skynet-types  │  │skynet-auth   │  │skynet-chat   │    │
│  │(Grundtypen)  │  │(Authentifika-│  │(Chat-Proto-  │    │
│  │              │  │ tionsprotokoll)│  │ koll)        │    │
│  └──────────────┘  └──────────────┘  └──────────────┘    │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐    │
│  │skynet-gateway│  │skynet-memory │  │skynet-service│    │
│  │(Gateway-Pro- │  │(Speicherpro- │  │(Dienstproto- │    │
│  │ tokoll)      │  │ tokoll)       │  │ koll)        │    │
│  └──────────────┘  └──────────────┘  └──────────────┘    │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐    │
│  │skynet-bridge │  │skynet-notific│  │skynet-persist│    │
│  │(Bridge-Pro-  │  │ation(Benach- │  │ence(Persis-  │    │
│  │ tokoll)      │  │ richtigung)   │  │ tenzprotokoll)│   │
│  └──────────────┘  └──────────────┘  └──────────────┘    │
└─────────────────────────────────────────────────────────────┘
```

### Beschreibung der Verantwortlichkeiten jeder Schicht

#### 1. Präsentationsschicht (Anwendungsseite)

Die Präsentationsschicht ist die Schnittstelle, mit der Benutzer direkt interagieren. Sie umfasst verschiedene Anwendungen und Frontend-Schnittstellen.

**Hauptkomponenten:**
- `ai-company` - Hauptanwendungs-Backend
- `ai-empire` - Empire-Modul-Backend
- `ai-planet` - Planet-Modul-Backend
- `ai-waifu` - Charakter-Modul-Backend
- Vue.js-Frontend-Anwendung

**Verantwortlichkeiten:**
- Bereitstellung der Benutzeroberfläche
- Verarbeitung von Benutzerinteraktionen
- Aufruf der Kernmodule der Implementierungsschicht
- Datenanzeige und -erfassung

#### 2. Implementierungsschicht (augur-* Kernmodule)

Die Implementierungsschicht ist die zentrale Geschäftslogikschicht des Systems und enthält mehrere spezialisierte Funktionsmodule.

**Hauptmodule:**
- `augur-agent` - Agentenverwaltung
- `augur-orchestrator` - Orchestrierung
- `augur-organization` - Organisationsverwaltung
- `augur-skill` - Fähigkeitsverwaltung
- `augur-memory` - Speichersystem
- `augur-persistence` - Persistenz
- `augur-file-system` - Dateisystem
- `augur-types` - Typdefinitionen

**Verantwortlichkeiten:**
- Implementierung der zentralen Geschäftslogik
- Koordination der Zusammenarbeit zwischen Modulen
- Verwaltung von Agenten und Fähigkeiten
- Bereitstellung von Funktionen für die Zusammenarbeit in Organisationen

#### 3. Protokollebene (Skynet-Protokoll)

Die Protokollebene bietet die zugrunde liegenden Kommunikations- und Datenaustauschprotokolle und ist die Infrastruktur des gesamten Systems.

**Hauptprotokolle:**
- `skynet-types` - Grundtypdefinitionen
- `skynet-auth` - Authentifizierungsprotokoll
- `skynet-chat` - Chat-Protokoll
- `skynet-gateway` - Gateway-Protokoll
- `skynet-memory` - Speicherprotokoll
- `skynet-service` - Dienstprotokoll
- `skynet-bridge` - Bridge-Protokoll
- `skynet-notification` - Benachrichtigungsprotokoll
- `skynet-persistence` - Persistenzprotokoll

**Verantwortlichkeiten:**
- Bereitstellung standardisierter Kommunikationsprotokolle
- Verarbeitung von Authentifizierung und Sicherheit
- Verwaltung von Datenspeicherung und -übertragung
- Unterstützung dezentraler Kommunikation

## Beziehung zu Skynet

Das Skynet-Protokoll bildet die Infrastrukturschicht (Protokollebene) des AI Company-Systems. Dieses Verzeichnis beschreibt die **Designkonzepte und Architekturprinzipien** von AI Company, während das [Skynet-Protokoll](../skynet/index.md) die **konkrete technische Implementierung** dieser Designkonzepte ist.

### Positionierung von Skynet in der Drei-Schichten-Architektur

In der Drei-Schichten-Architektur ist das Skynet-Protokoll die unterste Infrastrukturschicht:

```
Präsentationsschicht (Anwendungsseite)
    ↓ Aufruf
Implementierungsschicht (augur-* Kernmodule)
    ↓ Nutzung
Protokollebene (Skynet-Protokoll) ← Hier befindet sich Skynet
```

### Gesamtreferenzdiagramm

```
AI Company-Kernkonzepte (concepts/)
    ↓ Erstellt auf Basis von
Drei-Schichten-Architekturimplementierung (Präsentationsschicht + Implementierungsschicht)
    ↓ Basierend auf
Skynet-Protokoll (Protokollebene)
    ↓ Folgt
Dezentrales Designkonzept (architecture/)
```

### Zuordnung von AI Company-Konzepten zu Skynet-Konzepten

Die Kernkonzepte von AI Company bauen auf dem Skynet-Protokoll auf:

| AI Company-Konzept | Skynet-Entsprechung | Erklärung |
|--------------------|---------------------|-----------|
| **Organisation (Organization)** | **Subnetz (Subnet)** | Jede Organisation entspricht einem unabhängigen Subnetz |
| **Benutzer (echter menschlicher Benutzer)** | **Subnetz-Benutzer (user_id/auth_id)** | Echte Benutzer entsprechen Benutzeridentitäten innerhalb eines Subnetzes |
| **Adressbuch - interne Kontakte** | **user_id** | Für interne Kontakte reicht die user_id |
| **Adressbuch - externe Kontakte** | **subnet_id + user_id** | Für externe Kontakte wird das Tupel subnet_id + user_id benötigt |
| **Organisationsstruktur** | **AI Company-eigenständig** | Die Organisationsstruktur ist ein Konzept, das AI Company auf Subnetzen aufbaut |
| **Team (Agent Cluster)** | **AI Company-eigenständig** | Das Team ist ein eigenständiges Konzept von AI Company für die Zusammenarbeit von Agenten |
| **Projekt (Project)** | **AI Company-eigenständig** | Ein Projekt ist an einen Kanal (nicht austauschbar) und ein Team (austauschbar) gebunden und hat das Konzept eines Projektadministrators |
| **Kanal (Channel)** | **Kanal (Channel)** | Ein Kanal ist ein natives Chatgruppenkonzept von Skynet, das nach Bindung an ein Projekt nicht mehr ausgetauscht werden kann |
| **Aufgabe (Task)** | **AI Company-eigenständig** | Eine Aufgabe ist ein Konzept für Arbeitseinheiten innerhalb eines Projekts |
| **Mitarbeiter (AI Agent)** | **AI Company-eigenständig** | Ein AI Agent-Mitarbeiter ist ein Containerkonzept für eine Sammlung von Fähigkeiten |
| **Fähigkeit (Skill)** | **AI Company-eigenständig** | Eine Fähigkeit ist eine interne Arbeitseinheit eines Agenten |
| **Kapazität (Capability)** | **AI Company-eigenständig** | Eine Kapazität ist eine externe Erweiterungsfunktion eines Agenten (einschließlich API, MCP usw.) |
| **Arbeitsbereich (Workspace)** | **AI Company-eigenständig** | Der Arbeitsbereich ist ein eigenständiges Konzept von AI Company für verteilte Zusammenarbeit |
| **Arbeitsknoten (Worker Node)** | **Dienstknoten (Service Node)** | Ein Arbeitsknoten ist ein Dienstknoten von Skynet |
| **Arbeitsablauf (Workflow)** | **AI Company-eigenständig** | Der Arbeitsablauf ist ein eigenständiges Konzept von AI Company für die Orchestrierung von Aufgaben |
| **Termin/geplante Aufgabe (Schedule)** | **AI Company-eigenständig** | Ein Termin/eine geplante Aufgabe ist ein eigenständiges Konzept von AI Company für zeitgesteuerte Aufgaben |
| **Arbeitsbericht (Report)** | **AI Company-eigenständig** | Arbeitsberichte (Tages-, Wochen-, Monatsberichte) sind ein Konzept für die Zusammenfassung von Arbeitsergebnissen von AI Agents, Teams und Aufgaben |

### Zuordnung von Designkonzepten zu technischen Implementierungen

| Designkonzept | Skynet-Implementierung |
|--------------|-----------------------|
| Zero-Trust-Architektur | Knotenauthentifizierung, Ende-zu-Ende-Verschlüsselung |
| Dezentralisierung | Peer-to-Peer-Netzwerk, Zwei-Ebenen-Struktur aus Hauptnetz und Subnetzen |
| Überprüfbare Berechnung | Digitale Signaturen, Kreuzvalidierung durch mehrere Knoten |
| Datensouveränität | Ende-zu-Ende-Verschlüsselung, benutzergesteuerte Schlüssel |

Siehe [Skynet-Protokoll](../skynet/index.md) für die konkrete technische Implementierung und [Kernkonzepte](../../concepts/index.md) für die übergeordneten Konzepte von AI Company.
