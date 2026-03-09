# Implementierungsschicht (augur-* Kernmodule)

Dieses Dokument beschreibt detailliert die Kernmodule der Implementierungsschicht im AI Company-System. Die Implementierungsschicht liegt zwischen der Präsentationsschicht und der Protokollebene und stellt die Kernlogik und Funktionsimplementierung bereit.

## Überblick

Die Implementierungsschicht besteht aus mehreren unabhängigen `augur-*`-Modulen, von denen jedes für einen bestimmten Funktionsbereich verantwortlich ist. Diese Module arbeiten zusammen durch Dependency Injection und klare Schnittstellendesign und bieten eine solide Funktionsgrundlage für Anwendungen auf höherer Ebene.

## Modulliste

Die Implementierungsschicht enthält die folgenden Kernmodule:

| Modulname | Verantwortlichkeiten | Status |
|-----------|---------------------|--------|
| [augur-agent](#augur-agent) | Agentenverwaltung | Kern |
| [augur-orchestrator](#augur-orchestrator) | Task-Orchestrierung und Workflow-Management | Kern |
| [augur-organization](#augur-organization) | Verwaltung von Organisationen, Abteilungen und Rollen | Kern |
| [augur-skill](#augur-skill) | Skill-Plugins und Fähigkeitserweiterung | Kern |
| [augur-memory](#augur-memory) | Verwaltung des Gedächtnissystems | Kern |
| [augur-persistence](#augur-persistence) | Implementierung der Persistenzschicht | Infrastruktur |
| [augur-file-system](#augur-file-system) | Dateispeicherdienst | Infrastruktur |
| [augur-types](#augur-types) | Typdefinitionen und Fehlerbehandlung | Infrastruktur |

## Modulabhängigkeitsdiagramm

```
┌─────────────────────────────────────────────────────────────────┐
│                      Geschäftlogikschicht                         │
├─────────────────────────────────────────────────────────────────┤
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐          │
│  │augur-agent   │  │augur-orchestr│  │augur-organiz │          │
│  │(Agentenverwal│  │ator(Orchestra│  │ation(Organisa│          │
│  │tung)         │  │tor)           │  │tion)         │          │
│  └──────────────┘  └──────────────┘  └──────────────┘          │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐          │
│  │augur-skill   │  │augur-memory  │  │              │          │
│  │(Skillverwal  │  │(Gedächtnis-  │  │              │          │
│  │tung)         │  │system)        │  │              │          │
│  └──────────────┘  └──────────────┘  └──────────────┘          │
└─────────────────────────────────────────────────────────────────┘
                            ↓ Abhängigkeit
┌─────────────────────────────────────────────────────────────────┐
│                      Infrastrukturschicht                         │
├─────────────────────────────────────────────────────────────────┤
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐          │
│  │augur-persist │  │augur-file-sys│  │augur-types   │          │
│  │ence(Persist  │  │tem(Dateisyst │  │(Typdefinition│          │
│  │enz)          │  │em)            │  │)             │          │
│  └──────────────┘  └──────────────┘  └──────────────┘          │
└─────────────────────────────────────────────────────────────────┘
                            ↓ Nutzung
┌─────────────────────────────────────────────────────────────────┐
│                   Protokollebene (Skynet)                        │
└─────────────────────────────────────────────────────────────────┘
```

## Detaillierte Modulbeschreibung

### augur-types

**Verantwortlichkeiten:** Stellt gemeinsam genutzte Typdefinitionen für die Implementierungsschicht und einen einheitlichen Fehlerbehandlungsmechanismus bereit.

**Hauptfunktionen:**
- Definition von Kern-Datenstrukturen (Arbeitsbereiche, Projekte, Unternehmen, Teams, Mitarbeiter, Fähigkeiten usw.)
- Einheitliche Fehlerklasse `AugurError` und Fehlerbehandlungsmechanismus
- Bereitstellung von Standardaufzählungen (z. B. `ProjectStatus`, `RoleType`, `SkillLevel` usw.)
- Unterstützung für internationalisierte Fehlermeldungen (über i18n-Schlüssel)

**Kernstrukturen:**
- `Workspace` - Arbeitsbereich
- `Project` - Projekt
- `Company` - Unternehmen
- `Team` - Team
- `Employee` - Mitarbeiter
- `Skill` - Fähigkeit

**Abhängigkeiten:** Keine Abhängigkeiten zu anderen augur-*-Modulen, ist die Grundlage für alle anderen Module.

### augur-persistence

**Verantwortlichkeiten:** Stellt eine einheitliche Persistenzschichtimplementierung bereit, die mehrere Datenbank-Backends unterstützt.

**Hauptfunktionen:**
- Unterstützung mehrerer Datenbanken (SQLite, PostgreSQL)
- Einheitliches Repository-Schnittstellendesign
- Verwaltung von Datenbankmigrationen
- Unterstützung für Persistierung von Entitätstypen

**Unterstützte Entitätstypen:**
- Benutzer
- Organisation
- Abteilung
- Rolle
- Dialog
- Nachricht
- Agent
- Gedächtnis
- Gedächtnis-Tag

**Verwendungsbeispiel:**
```rust
use augur_persistence::{AugurPersistence, PersistenceConfig, DatabaseType};

let config = PersistenceConfig::default();
let persistence = AugurPersistence::new(config).await?;

let user_repo = persistence.user_repository();
```

**Abhängigkeiten:** Abhängig von `augur-types`, wird von Geschäftslogikmodulen abhängig.

### augur-file-system

**Verantwortlichkeiten:** Stellt einen Dateispeicherdienst basierend auf dem lokalen Dateisystem bereit.

**Hauptfunktionen:**
- Dateiupload und -download
- Verwaltung von Dateimetadaten
- Steuerung des Dateizugriffsebenen
- Dateisuche und -filterung
- Liste von Benutzer- und Organisationsdateien
- Dateikopier- und Verschiebevorgänge
- Unterstützung für mehrere MIME-Typ-Erkennungen

**Verwendungsbeispiel:**
```rust
use augur_file_system::{FsFileService, FsFileServiceConfig};

let config = FsFileServiceConfig::default();
let service = FsFileService::new(config).await?;
```

**Abhängigkeiten:** Abhängig von `augur-types`, wird von Geschäftslogikmodulen abhängig.

### augur-memory

**Verantwortlichkeiten:** Implementiert das Gedächtnissystem, das Erstellung, Speicherung, Abfrage und Verwaltung von Gedächtnissen unterstützt.

**Hauptfunktionen:**
- Gedächtnis-CRUD-Operationen
- Verwaltung von Gedächtnis-Tags
- Verwaltung von Gedächtnisbeziehungen
- Schlüsselwortsuche
- Filterung nach Tags und Zeitraum
- Import und Export von Gedächtnissen
- Extraktion von Dialogkontext
- Erstellung von Gedächtnissen aus Dialogen

**Verwendungsbeispiel:**
```rust
use augur_memory::AugurMemory;
use std::sync::Arc;

let memory_service = AugurMemory::new(
    Arc::new(memory_repository),
    Arc::new(memory_tag_repository),
);
```

**Abhängigkeiten:** Abhängig von `augur-types`, `augur-persistence`, wird von `augur-agent` usw. abhängig.

### augur-agent

**Verantwortlichkeiten:** Stellt Typdefinitionen und Schnittstellen für Agenten (Agent) bereit und verwaltet den Lebenszyklus von Agenten.

**Hauptfunktionen:**
- Agenten-Typdefinitionen
- Verwaltung des Agentenstatus
- Agenten-Konfigurationsschnittstelle
- Verwaltung des Agenten-Lebenszyklus

**Abhängigkeiten:** Abhängig von `augur-types`, `augur-memory`, `augur-skill`, wird von Anwendungen auf höherer Ebene abhängig.

### augur-orchestrator

**Verantwortlichkeiten:** Stellt Typen und Schnittstellen für Task-Orchestrierung und Workflow-Management bereit.

**Hauptfunktionen:**
- Task-Orchestrierungs-Schnittstelle
- Workflow-Management
- Task-Planung
- Abhängigkeitsverwaltung

**Abhängigkeiten:** Abhängig von `augur-types`, wird von Anwendungen auf höherer Ebene abhängig.

### augur-organization

**Verantwortlichkeiten:** Stellt Typen und Schnittstellen für die Verwaltung von Organisationen, Abteilungen und Rollen bereit.

**Hauptfunktionen:**
- Organisationsverwaltung
- Abteilungsstruktur
- Rollenberechtigungen
- Mitgliederverwaltung

**Abhängigkeiten:** Abhängig von `augur-types`, `augur-persistence`, wird von Anwendungen auf höherer Ebene abhängig.

### augur-skill

**Verantwortlichkeiten:** Stellt Typen und Schnittstellen für Skill-Plugins und Fähigkeitserweiterung bereit.

**Hauptfunktionen:**
- Skill-Plugin-Schnittstelle
- Skill-Definition
- Skill-Ausführung
- Plugin-Management

**Abhängigkeiten:** Abhängig von `augur-types`, wird von `augur-agent` abhängig.

## Beispiele für die Zusammenarbeit zwischen Modulen

### Beispiel 1: Agentenausführung von Aufgaben

1. Anwendung auf höherer Ebene ruft `augur-orchestrator` auf, um eine Aufgabe zu initiieren
2. `augur-orchestrator` sucht nach verfügbaren `augur-agent`-Agenten
3. `augur-agent` lädt die erforderlichen Fähigkeiten mit `augur-skill`
4. Während der Ausführung zeichnet `augur-agent` mit `augur-memory` Gedächtnisse auf und fragt sie ab
5. Alle Statusänderungen werden über `augur-persistence` gespeichert

### Beispiel 2: Organisationszusammenarbeit

1. Anwendung auf höherer Ebene verwaltet die Organisationsstruktur über `augur-organization`
2. `augur-organization` speichert Organisationsdaten mit `augur-persistence`
3. Benutzerdateien werden über `augur-file-system` verwaltet
4. Agenten innerhalb der Organisation arbeiten über `augur-agent` zusammen

## Designprinzipien

Die Implementierungsschicht folgt den folgenden Designprinzipien:

1. **Single Responsibility Principle** - Jedes Modul ist nur für einen klar definierten Funktionsbereich verantwortlich
2. **Dependency Inversion Principle** - Module auf höherer Ebene hängen nicht von Modulen auf niedrigerer Ebene ab, beide hängen von Abstraktionen ab
3. **Interface Segregation Principle** - Bietet minimale Schnittstellen, vermeidet fette Schnittstellen
4. **Modulares Design** - Module arbeiten zusammen über klare Grenzen und Abhängigkeiten
5. **Testbarkeit** - Jedes Modul sollte leicht unabhängig testbar sein

## Erweiterungsanleitung

Wenn Sie neue Funktionen hinzufügen möchten, befolgen Sie die folgenden Schritte:

1. Bestimmen Sie, zu welchem vorhandenen Modul die Funktion gehört oder ob ein neues Modul erstellt werden muss
2. Definieren Sie in `augur-types` die erforderlichen Datenstrukturen und Fehlerklassen
3. Implementieren Sie die Kernlogik im entsprechenden Geschäftsmodul
4. Wenn Persistenz erforderlich ist, fügen Sie in `augur-persistence` ein Repository hinzu
5. Wenn Dateispeicherung erforderlich ist, verwenden Sie `augur-file-system`
6. Aktualisieren Sie dieses Dokument, um eine Beschreibung der neuen Funktionen hinzuzufügen

## Verwandte Dokumentation

- [Architekturübersicht](./index.md)
- [Dezentrales Design](./decentralization.md)
- [Sicherheitsmodell](./security-model.md)
- [Datenmodelle](../data-models.md)
- [Skynet-Protokoll](../skynet/index.md)
