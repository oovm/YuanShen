# Protokollebene Design

Dieses Dokument beschreibt detailliert die Protokollebene des AI Company-Systems, d.h. die Skynet-Protokollmodule im Verzeichnis protocols/. Die Protokollebene ist die Infrastruktur des gesamten Systems und bietet standardisierte Kommunikationsprotokolle, Datentypdefinitionen und Interaktionsschnittstellen.

## Übersicht der Protokollebene

Die Protokollebene ist die unterste Ebene der dreischichtigen Architektur von AI Company und bietet Basisdienste und standardisierte Interaktionsprotokolle für die obere Implementierungsebene (augur-*-Kernmodule) und die Präsentationsebene.

### Rolle der Protokollebene in der Architektur

```
┌─────────────────────────────────────────────────────────────┐
│                   Präsentationsebene (Anwendungsseite)       │
├─────────────────────────────────────────────────────────────┤
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐    │
│  │  ai-company  │  │  ai-empire   │  │  ai-planet   │    │
│  └──────────────┘  └──────────────┘  └──────────────┘    │
└─────────────────────────────────────────────────────────────┘
                            ↓ Aufrufen
┌─────────────────────────────────────────────────────────────┐
│              Implementierungsebene (augur-*-Kernmodule)      │
├─────────────────────────────────────────────────────────────┤
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐    │
│  │augur-agent   │  │augur-orchestr│  │augur-organiz │    │
│  └──────────────┘  └──────────────┘  └──────────────┘    │
└─────────────────────────────────────────────────────────────┘
                            ↓ Nutzen
┌─────────────────────────────────────────────────────────────┐
│              ← Protokollebene (Skynet-Protokoll) →           │
├─────────────────────────────────────────────────────────────┤
│  Basis-Typen  Authentifizierung  Chat  Gateway  Speicher  │
│  Brücke  Benachrichtigung  Persistenz  Service             │
└─────────────────────────────────────────────────────────────┘
```

### Kernverantwortlichkeiten

Die Kernverantwortlichkeiten der Protokollebene umfassen:

1. **Bereitstellung standardisierter Datentypdefinitionen**: Einheitliches Datenaustauschformat zwischen Modulen
2. **Definition von Kommunikationsprotokollen**: Standardisierung der Interaktionsweise zwischen Modulen und Diensten
3. **Bereitstellung grundlegender Sicherheitsmechanismen**: Identitätsauthentifizierung, verschlüsselte Übertragung, Zugriffssteuerung
4. **Unterstützung dezentraler Kommunikation**: Zwei-Ebenen-Struktur (Hauptnetz-Subnetz), Peer-to-Peer-Kommunikation
5. **Abstraktion von Unterschieden der unteren Ebene**: Bereitstellung einheitlicher Schnittstellen für die obere Ebene, Maskierung von Implementierungsdetails der unteren Ebene

## Detaillierte Erklärung der Protokollmodule

### 1. skynet-types - Basis-Typdefinitionen

**Verantwortlichkeiten und Funktionen**:
- Bereitstellung aller gemeinsamen Typdefinitionen der Skynet-Protokollebene
- Definition von Kern-Datenstrukturen, Enums und Konstanten
- Bereitstellung von Mechanismen zur Generierung und Verarbeitung von Identifikatoren
- Definition von Fehlertypen und einheitlicher Fehlerbehandlungsspezifikationen
- Bereitstellung von JSON-RPC- und WebSocket-Nachrichtentypen

**Hauptuntermodule**:
- `agent` - Agentenbezogene Typen
- `bridge` - Brückenbezogene Typen
- `chat` - Chatbezogene Typen
- `error` - Fehlerbezogene Typen
- `id` - Identifikatorbezogene Typen
- `jsonrpc` - JSON-RPC-bezogene Typen
- `memory` - Gedächtnisbezogene Typen
- `org` - Organisationsbezogene Typen
- `resource` - Ressourcenbezogene Typen
- `subnet` - Subnetzbezogene Typen
- `user` - Benutzerbezogene Typen
- `utils` - Hilfsfunktionen
- `websocket` - WebSocket-bezogene Typen

**Rolle in der Architektur**: Als Basis für alle anderen Protokollmodule, bietet ein einheitliches Typsystem und gewährleistet die Datenkompatibilität zwischen den Modulen.

---

### 2. skynet-auth - Authentifizierungsprotokoll

**Verantwortlichkeiten und Funktionen**:
- Bereitstellung von Schnittstellen zur Benutzeridentitätsauthentifizierung
- Verwaltung von Rechten und Zugriffssteuerung
- Verarbeitung von Sitzungsverwaltung und Tokens
- Unterstützung mehrerer Authentifizierungsmethoden

**Kernfunktionen**:
- Definition des Benutzerauthentifizierungsablaufs
- Rechteverwaltung und Autorisierungsmechanismen
- Generierung und Überprüfung von Sitzungstokens
- Verwaltung der Token-Lebenszyklen

**Rolle in der Architektur**: Bietet eine sichere Basis für Identitätsauthentifizierung und Zugriffssteuerung für das gesamte System und gewährleistet, dass nur autorisierte Benutzer und Dienste auf Systemressourcen zugreifen können.

---

### 3. skynet-chat - Chat-Protokoll

**Verantwortlichkeiten und Funktionen**:
- Bereitstellung von Echtzeit-Chat und Sitzungsverwaltung
- Definition von Nachrichtentypen und -formaten
- Unterstützung von Chatverlauf
- Bereitstellung von Echtzeit-Kommunikationsschnittstellen

**Kernfunktionen**:
- Sitzungs- (Kanal-)Verwaltung
- Definition mehrerer Nachrichtentypen (Text, Datei, Bild usw.)
- Echtzeit-Nachrichtenversand
- Abfrage und Suche des Chatverlaufs

**Rolle in der Architektur**: Bietet grundlegende Kommunikationsunterstützung für die Kollaborationsfunktionen von AI Company, einschließlich Projektkanäle, Teamkommunikation usw.

---

### 4. skynet-gateway - Gateway-Protokoll

**Verantwortlichkeiten und Funktionen**:
- Bereitstellung von API-Gateway-Schnittstellen
- Verarbeitung von Anforderungsrouting und -weiterleitung
- Implementierung von Lastverteilung
- Ausführung von Sicherheitsüberprüfungen

**Kernfunktionen**:
- Anforderungsrouting und -verteilung
- Dienstsuche und Lastverteilung
- Anforderungsüberprüfung und Sicherheitsfilterung
- API-Versionsverwaltung

**Rolle in der Architektur**: Als Einstiegspunkt des Systems, einheitliche Verwaltung externer Anforderungen und Bereitstellung eines sicheren und effizienten API-Zugriffskanals.

---

### 5. skynet-memory - Gedächtnisprotokoll

**Verantwortlichkeiten und Funktionen**:
- Bereitstellung von Schnittstellen zur Speicherung und Abfrage von Gedächtnisinhalten
- Definition von Gedächtnisdatentypen
- Unterstützung von Gedächtnis-Suche und -Abfrage
- Verwaltung von Gedächtnis-Tags und -Klassifizierungen

**Kernfunktionen**:
- Speicherung und Abfrage von Gedächtnisdaten
- Vektorsuche im Gedächtnis
- Tag-Verwaltung und Klassifizierung
- Gedächtnisassoziation und Kontext

**Rolle in der Architektur**: Bietet Unterstützung für das Gedächtnissystem von Agenten, sodass Agenten historische Interaktionen speichern, Erfahrungen lernen und Langzeitgedächtnis aufbauen können.

---

### 6. skynet-service - Dienstprotokoll

**Verantwortlichkeiten und Funktionen**:
- Bereitstellung von allgemeinen Dienst-Schnittstellendefinitionen
- Unterstützung von Dienstsuchmechanismen
- Implementierung von Gesundheitsprüfung und -überwachung
- Verwaltung von Dienstregistrierung und -abbestellung

**Kernfunktionen**:
- Einheitliche Dienst-Schnittstellenspezifikation
- Dienstregistrierung und -suche
- Überwachung des Gesundheitszustands
- Verwaltung der Dienst-Lebenszyklen

**Rolle in der Architektur**: Bietet einheitliche Mechanismen zur Registrierung, Suche und Kommunikation für verschiedene Dienste im System und unterstützt die Microservices-Architektur.

---

### 7. skynet-bridge - Brückenprotokoll

**Verantwortlichkeiten und Funktionen**:
- Bereitstellung von Schnittstellen zur Integration externer Systeme
- Verarbeitung von Nachrichtenweiterleitung und Protokollumwandlung
- Unterstützung von Multi-Plattform-Integration
- Verwaltung der Kommunikation zwischen Systemen

**Kernfunktionen**:
- Brückenschaltung zu externen Systemen
- Protokollumwandlung und -anpassung
- Nachrichtenrouting und -weiterleitung
- Multi-Plattform-Connector

**Rolle in der Architektur**: Erlaubt AI Company, mit externen Systemen (wie Drittanbieter-Diensten, Legacy-Systemen usw.) zu integrieren und zu kommunizieren.

---

### 8. skynet-notification - Benachrichtigungsprotokoll

**Verantwortlichkeiten und Funktionen**:
- Bereitstellung von Nachrichtenbenachrichtigungs- und Push-Diensten
- Definition von Benachrichtigungstypen und -formaten
- Unterstützung mehrerer Push-Kanäle
- Verwaltung des Benachrichtigungsverlaufs

**Kernfunktionen**:
- Benachrichtigungsdienst-Schnittstelle
- Multi-Kanal-Push (Mobilgeräte, Desktop, E-Mail usw.)
- Definition von Benachrichtigungstypen
- Verwaltung des Benachrichtigungsverlaufs und des Status

**Rolle in der Architektur**: Bietet Echtzeit-Benachrichtigungsfähigkeiten für das System und gewährleistet, dass Benutzer wichtige Informationen und Aktualisierungen rechtzeitig erhalten.

---

### 9. skynet-persistence - Persistenzprotokoll

**Verantwortlichkeiten und Funktionen**:
- Bereitstellung von Data Warehouse- und Persistenzschnittstellen
- Definition von Entitätstypen und Repository-Mustern
- Unterstützung von Datenabfrage und Transaktionsverwaltung
- Bereitstellung einer einheitlichen Datenzugriffsschicht

**Kernfunktionen**:
- Definition von Repository-Schnittstellen
- Spezifikation von Entitätstypen
- Datenabfrage und -filterung
- Transaktionsverwaltung und Konsistenzgarantie

**Rolle in der Architektur**: Bietet eine einheitliche Persistenzabstraktion für das System, maskiert Unterschiede zwischen Datenbanken der unteren Ebene und unterstützt mehrere Speicherbackends.

---

### 10. skynet-storage - Speicherprotokoll

**Verantwortlichkeiten und Funktionen**:
- Bereitstellung von allgemeinen Speicherdienst-Schnittstellen
- Unterstützung mehrerer Speichertypen (Datei, Blob, Objekt usw.)
- Verwaltung von Speicherberechtigungen und Zugriffssteuerung
- Definition von Speicheroperationsspezifikationen

**Kernfunktionen**:
- Allgemeine Speicherdienst-Schnittstelle
- Unterstützung mehrerer Speichertypen
- Berechtigungssteuerung und Zugriffsverwaltung
- Definition von Speicheroperationen (Hochladen, Herunterladen, Löschen usw.)

**Rolle in der Architektur**: Bietet eine einheitliche Datei- und Objektspeicherabstraktion für das System und unterstützt verschiedene Speicheranforderungen.

## Designprinzipien der Protokollebene

### 1. Leichtgewichtigkeit und Flexibilität

Die Protokollebene definiert nur **mindestens notwendige** Schnittstellen und Typen und behält Leichtgewichtigkeit und Flexibilität bei. Spezifische Implementierungsdetails werden von den oberen Modulen entsprechend den Anforderungen autonom implementiert.

### 2. Standardisierung und Konsistenz

Alle Protokollmodule folgen einheitlichen Design-Spezifikationen und Namenskonventionen, um Konsistenz und Interoperabilität zwischen den Modulen zu gewährleisten.

### 3. Erweiterbarkeit

Das Protokolldesign bietet Erweiterungspunkte und unterstützt die Hinzufügung und Aktualisierung zukünftiger Funktionen, ohne vorhandene Schnittstellen zu beschädigen.

### 4. Sicherheit zuerst

Die Protokollebene integriert grundlegende Sicherheitsmechanismen, einschließlich Identitätsauthentifizierung, verschlüsselter Übertragung, Zugriffssteuerung usw., um die Systemsicherheit zu gewährleisten.

### 5. Dezentralisierungsunterstützung

Die Protokollebene unterstützt eine Zwei-Ebenen-Struktur (Hauptnetz-Subnetz) und Peer-to-Peer-Kommunikation und bietet Infrastruktur für dezentrale Anwendungen.

## Interaktionsbeziehungen mit der oberen Ebene

### Implementierungsebene (augur-*-Module)

Die Kernmodule der Implementierungsebene (wie augur-agent, augur-orchestrator usw.) nutzen direkt die von der Protokollebene bereitgestellten Schnittstellen und Typen:

- `augur-agent` nutzt `skynet-types` und `skynet-memory`
- `augur-organization` nutzt `skynet-types` und `skynet-persistence`
- `augur-orchestrator` nutzt `skynet-service` und `skynet-gateway`

### Präsentationsebene (Anwendungsseite)

Die Präsentationsebene nutzt die Protokollebene indirekt über die Implementierungsebene oder in einigen Fällen direkt die Basistypen und -schnittstellen der Protokollebene.

## Zusammenfassung

Die Protokollebene ist die Infrastruktur des AI Company-Systems und bietet durch die Bereitstellung standardisierter Kommunikationsprotokolle, Datentypdefinitionen und Interaktionsschnittstellen eine solide Basis für die obere Implementierungsebene und Präsentationsebene. Jede Protokollmodul hat seine eigenen Aufgaben und bildet gemeinsam ein vollständiges, flexibles und sicheres Protokollsystem, das die dezentrale Design- und Unternehmensanwendungsanforderungen des Systems unterstützt.

Durch die Abstraktion der Protokollebene erreicht AI Company:
- Modulares Design, wobei jede Komponente unabhängig weiterentwickelt werden kann
- Standardisierte Schnittstellen zur einfachen Integration und Erweiterung
- Sicherheitsgrundlage zur Gewährleistung der Systemsicherheit
- Dezentralisierungsunterstützung zur Umsetzung von Datensouveränität und Benutzerkontrolle