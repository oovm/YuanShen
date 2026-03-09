# Sicherheitsmodell

Dieses Dokument beschreibt, wie AI Company als Subnetz-Implementierung auf dem Skynet-Protokoll Sicherheitsmechanismen auf Architekturebene ergänzt.

## Überblick

### Schichtweise Verantwortungszuteilung

| Schicht | Verantwortlicher Inhalt | Erklärung |
|---------|-------------------------|-----------|
| **Skynet-Protokollschicht** | Netzwerkkommunikation, grundlegende kryptografische Primitive, Subnetzabstraktion | Leichtgewichtig, flexibel, universell |
| **Subnetz-Implementierungsschicht (AI Company)** | Szenariospezifische Sicherheitsmechanismen, Client-Implementierungsleitfaden | Je nach Subnetztyp bedarfsgerecht implementiert |

Das Skynet-Protokoll definiert nur **minimal notwendige** Sicherheitsmechanismen, um Leichtgewichtigkeit und Flexibilität zu gewährleisten. Spezifische Subnetze (wie AI Company) ergänzen je nach Bedarf zusätzliche Sicherheitsmaßnahmen auf Architekturebene.

---

## 1. Metadatenschutz

### Position der Skynet-Protokollschicht
Das Skynet-Protokoll **bietet keinen** Metadatenschutz, um die Leichtgewichtigkeit des Protokolls zu gewährleisten.

### Verantwortlich für die Implementierung durch die AI Company Subnetzschicht

Für unternehmensweite Anwendungen werden Metadatenschutzlösungen von AI Company entworfen und implementiert:

#### Basismaßnahmen (empfohlen)
| Maßnahme | Erklärung |
|----------|-----------|
| **Routing-Randomisierung** | Clients wählen Serviceknoten zufällig aus |
| **Reduzierte Zeitstempelgenauigkeit** | Sekundengenauigkeit (statt Millisekunden) |

#### Langfristige Planung: Mixnetz
Wenn starker Metadatenschutz benötigt wird, kann AI Company in zukünftigen Versionen ein Mixnetz (wie Loopix) einführen, was jedoch erfordert:
- Signifikante Architekturänderungen
- Akzeptanz höherer Latenz und Bandbreitenaufwand
- Nur für Szenarien mit extrem hohen Datenschutzanforderungen geeignet

---

## 2. Migration zur Quantenresistenten Kryptographie (PQC)

### Position der Skynet-Protokollschicht
Das Skynet-Protokoll **reservert Erweiterungspunkte** zur Unterstützung der Migration zu PQC-Algorithmen, erfordert dies jedoch nicht zwingend.

### Verantwortlich für die Implementierung durch die AI Company Subnetzschicht

#### 2.1 Hybrid-Zertifikatmodus
AI Company ist verantwortlich für die Implementierung der gleichzeitigen Unterstützung klassischer Kryptographie und Post-Quantum-Kryptographie:
- **Doppel-Schlüsselpaare**: Jede Identität hält gleichzeitig ein Ed25519- und ein CRYSTALS-Dilithium-Schlüsselpaar
- **Doppel-Signaturen**: Wichtige Operationen werden gleichzeitig mit beiden Algorithmen signiert
- **Schrittweise Migration**: Zuerst Validierung unterstützen, dann schrittweise auf PQC als Hauptalgorithmus umstellen

#### 2.2 PQC-Kryptosuite
| Komponente | Klassischer Algorithmus | PQC-Algorithmus | Hybridmodus |
|------------|--------------------------|-----------------|-------------|
| **Signatur** | Ed25519 | CRYSTALS-Dilithium | Ed25519 + Dilithium |
| **Schlüsselverkapselung** | X25519 | CRYSTALS-Kyber | X25519 + Kyber |
| **Hash** | Blake3 | Blake3 | Blake3 |

#### 2.3 Schlüsselrotationsstrategie
- **Notfallrotation**: Bei Entdeckung einer Quantenbedrohung kann sofort eine netzweite Schlüsselrotation ausgelöst werden
- **Regelmäßige Rotation**: Standardmäßig alle 90 Tage eine Rotation langfristiger Schlüssel
- **Forward Secrecy**: Sitzungsschlüssel verwenden PQC KEM zur Gewährleistung von Forward Secrecy

### Migrationsroadmap
```
Phase 1 (aktuell): Klassische Kryptographie als Hauptverfahren
    ↓
Phase 2: Hybridmodus (gleichzeitige Unterstützung klassisch und PQC)
    ↓
Phase 3: PQC als Hauptverfahren, klassisch als Alternative
    ↓
Phase 4: Volle PQC (Entfernen der Unterstützung für klassische Algorithmen)
```

---

## 3. Client-Sicherheitsimplementierungsleitfaden (AI Company Subnetzschicht)

### Position der Skynet-Protokollschicht
Das Skynet-Protokoll **definiert Schnittstellen**, überlässt die konkrete Implementierung jedoch Clients und der Subnetzschicht.

### Spezifikationen der AI Company Subnetzschicht

#### 3.1 Schlüsselverwaltungsspezifikationen

##### Schlüsselspeicherhierarchie
| Schlüsseltyp | Speicherort | Verschlüsselungsschutz |
|--------------|-------------|------------------------|
| **Identitäts-Privatschlüssel** | System-Schlüsselkette / HSM / TEE | Hardware-Schutz |
| **Geräte-Privatschlüssel** | Verschlüsselter lokaler Speicher | Vom Benutzerpasswort abgeleiteter Schlüssel |
| **Sitzungsschlüssel** | Arbeitsspeicher (Prozessisolation) | Automatische Zerstörung |

##### Anforderungen an die Schlüsselableitung
- **Algorithmus**: Pflichtmäßige Verwendung von Argon2id
- **Parameter**:
  - Arbeitsspeicher: mindestens 64MB
  - Iterationen: mindestens 3-mal
  - Parallelitätsgrad: 4
  - Salt: 16 Byte CSPRNG-generiert

##### Schlüsselbackup und -wiederherstellung
- **Shamir-Geheimnisverteilung**: Wiederherstellungsschlüssel in 5 Teile aufteilen, 3 Teile reichen zur Wiederherstellung
- **Offlinespeicherung**: Benutzer werden ermutigt, Wiederherstellungsschlüssel zu drucken oder auf Offline-Geräten zu speichern
- **Soziale Wiederherstellung**: Optionale Unterstützung vertrauenswürdiger Kontakte zur Unterstützung der Wiederherstellung

#### 3.2 API-Key-Verwaltungsspezifikationen (AI Company spezifisch)

##### API-Key-Sicherheitsprinzipien
| Prinzip | Erklärung |
|---------|-----------|
| **Minimale Rechte** | Jedem API-Key werden nur die minimal erforderlichen Rechte für die Aufgabe gewährt |
| **Kurzlebigkeit** | Vorzugsweise kurzfristige Tokens statt permanenter API-Keys verwenden |
| **Isolierte Nutzung** | Verschiedene Agenten/Dienste verwenden verschiedene API-Keys |
| **Widerrufbarkeit** | Kompromittierte oder nicht mehr benötigte API-Keys können jederzeit widerrufen werden |

##### API-Key-Speicherung
- **Speicherort**: System-Schlüsselkette / Sicherer Schlüsselverwaltungsdienst (KMS)
- **Verschlüsselungsschutz**: Statische Verschlüsselung, API-Key mit Hauptschlüssel verschlüsseln
- **Speicherschutz**: Sofort aus dem Arbeitsspeicher löschen nach Gebrauch, um Austausch auf die Festplatte zu vermeiden

##### API-Key-Lebenszyklusverwaltung
```
Erstellen → Verteilen → Verwenden → Rotieren → Widerrufen → Zerstören
```

**Erstellen**:
- Verwendung von CSPRNG zur Generierung eines ausreichend starken API-Keys (mindestens 256 Bit Entropie)
- Aufzeichnung von Erstellungszeit, Ersteller, Zweck, Rechtebereich

**Verteilen**:
- Übertragung über sicheren Kanal (Ende-zu-Ende-Verschlüsselung)
- Vermeidung der Offenlegung des vollständigen API-Keys in Protokollen, Fehlermeldungen
- Nur die ersten/letzten Zeichen des Keys anzeigen zur Identifizierung

**Verwenden**:
- Einfügen über Umgebungsvariablen oder sichere Konfigurationsdateien statt festcodiert
- Übertragung über Anfrageheader oder Autorisierungsfelder (z. B. `Authorization: Bearer <key>`)
- Vermeidung der Übertragung in URL-Abfrageparametern

**Rotieren**:
- Regelmäßige Rotation (Standardmäßig alle 90 Tage)
- Unterstützung einer Übergangsperiode, in der alte und neue Keys gleichzeitig funktionieren (mindestens 7 Tage)
- Sofortige Rotation bei Sicherheitsvorfällen

**Widerrufen**:
- Bereitstellung einer Self-Service-Widerrufsschnittstelle
- Sofortige Ungültigmachung nach Widerruf
- Aufzeichnung von Widerrufgrund und -zeit

**Zerstören**:
- Löschen aus allen Speicherorten
- Überschreiben von Resten im Arbeitsspeicher und auf der Festplatte
- Überprüfung der vollständigen Zerstörung

##### Notfallantwort bei API-Key-Kompromittierung
1. **Sofort widerrufen**: Bei Entdeckung der Kompromittierung sofort widerrufen
2. **Audit-Protokoll prüfen**: Alle Verwendungsaufzeichnungen dieses Keys prüfen
3. **Auswirkungsbewertung**: Bewertung möglicher Verluste durch die Kompromittierung
4. **Benutzer benachrichtigen**: Benachrichtigung der betroffenen Benutzer/Teams
5. **Ursachenanalyse**: Finden der Ursache der Kompromittierung, um Wiederholungen zu verhindern
6. **Sicherheitsverbesserung**: Verbesserung von Speicher- und Nutzungsweise

##### Vorzug von temporären Tokens
Für Agent-Szenarien vorzugsweise kurzfristige Tokens statt permanenter API-Keys verwenden:
- **OAuth 2.0**: Verwendung von Client Credentials oder Authorization Code Flow
- **JWT**: Kurzfristige JWT-Tokens (Gültigkeitsdauer < 1 Stunde)
- **Dynamische Aktualisierung**: Verwendung von Refresh Token zur Erlangung eines neuen Access Tokens

#### 3.3 Zufallszahlengenerierungsspezifikationen

##### Anforderungen an die Entropiequelle
- **Hauptentropiequelle**: Betriebssystem-CSPRNG (Windows: `CryptGenRandom`, Linux: `/dev/urandom`)
- **Zusätzliche Entropiequellen**:
  - Gerätesensordaten (Beschleunigungsmesser, Gyroskop)
  - Zeitstempel von Benutzerinteraktionen
  - Netzwerklatenzmessungen
- **Entropiemischung**: Verwendung von Fortuna- oder Yarrow-Algorithmus zum Mischen von Mehrquellenentropie

##### CSPRNG-Validierung
- Implementierung des NIST SP 800-90A-Standards
- Regelmäßige Durchführung von Zufallstests (Dieharder, TestU01)
- Verwendung nicht kryptographisch sicherer Zufallszahlengeneratoren verboten

#### 3.4 Referenzimplementierung (AI Company Subnetz-SDK)
Bereitstellung eines offiziellen SDKs mit:
- **Kryptobibliotheks-Kapselung**: Einheitliche Schnittstelle für kryptografische Primitive
- **Schlüsselverwaltungsmodul**: Einsatzbereite sichere Schlüsselspeicherung
- **API-Key-Verwaltungsmodul**: Sichere API-Key-Lebenszyklusverwaltung

---

## 4. Sicherheitsaudit und Protokollierung (AI Company verantwortlich)

### Position der Skynet-Protokollschicht
Audit-Protokolle **sind nicht Teil des Skynet-Protokolls** und werden vollständig von den Subnetz-Erstellern je nach Bedarf autonom implementiert.

### Grund
Die Audit-Anforderungen verschiedener Subnetztypen unterscheiden sich extrem:

| Subnetztyp | Audit-Anforderungen | Beispiel |
|------------|---------------------|----------|
| **Organisations-Subnetz (AI Company)** | Hohe Anforderungen | Vollständige Audit-Protokolle, unveränderlich, mehrjährige Aufbewahrung, Compliance-Zertifizierung |
| **Community-Subnetz** | Mittlere Anforderungen | Grundlegende Betriebsprotokolle, optionale Aufbewahrung |
| **Privates Subnetz** | Niedrige Anforderungen | Möglicherweise keine Audit-Protokolle erforderlich |

### Verantwortlich für die Implementierung durch die AI Company Subnetzschicht

AI Company als unternehmensweites Subnetz ist verantwortlich für die Implementierung einer vollständigen Audit-Protokollfunktion:

#### 4.1 Audit-Ereignistypen
- Identitätsauthentifizierungsereignisse
- Schlüssel-/API-Key-Operationen
- Rechteänderungen
- Zugriff auf sensible Ressourcen

#### 4.2 Protokollsicherheitsanforderungen
- **Lokale Client-Speicherung**: Audit-Protokolle werden von Clients gesammelt und gespeichert
- **Verschlüsselungsschutz**: Sensible Audit-Protokolle verschlüsselt speichern
- **Integritätsschutz**: Verwendung digitaler Signaturen zur Verhinderung von Manipulationen
- **Subnetzweite Konfiguration**: Subnetz-Ersteller können Audit-Richtlinien definieren

---

## 5. Sybil-Angriffsschutz (AI Company verantwortlich)

### Position der Skynet-Protokollschicht
Das Skynet-Protokoll **erfordert nicht zwingend** Sybil-Angriffsschutz, um die Flexibilität des Protokolls zu gewährleisten.

### Verantwortlich für die Implementierung durch die AI Company Subnetzschicht

#### 5.1 Mehrschichtiges Schutzsystem

| Schutzebene | Mechanismus | Erklärung |
|-------------|-------------|-----------|
| **Erste Ebene** | Identitätsauthentifizierung | E-Mail, Mobiltelefonnummer, WebAuthn usw. |
| **Zweite Ebene** | Proof-of-Work | Hashcash oder ähnliche Mechanismen |
| **Dritte Ebene** | Sozialgraph-Validierung | Unterstützung durch vertrauenswürdige Kontakte |
| **Vierte Ebene** | Wirtschaftsliche Sicherung | Sperren von Tokens als Garantie |

#### 5.2 Identitätsauthentifizierungsoptionen
- **WebAuthn/FIDO2**: Empfohlene starke Authentifizierungsmethode
- **E-Mail-Validierung**: Senden eines Validierungslinks
- **Mobiltelefonnummer-Validierung**: SMS-Validierungscode
- **OAuth**: Unterstützung von Drittanbieter-Identitätsanbietern (Google, GitHub usw.)

#### 5.3 Reputationssystem
- **Anfängliche Reputation**: Neue Benutzer haben niedrige Reputation, Betriebsfrequenz eingeschränkt
- **Reputationsaufbau**: Normaler Gebrauch erhöht schrittweise die Reputation
- **Reputationsstrafen**: Bösartiges Verhalten senkt die Reputation
- **Subnetzübergreifende Reputation**: Reputation kann zwischen verschiedenen Subnetzen geteilt werden

---

## 6. Forward Secrecy-Verstärkung (Subnetzschicht-Implementierungsleitfaden)

### Position der Skynet-Protokollschicht
Das Skynet-Protokoll **verwendet** Double Ratchet und MLS zur Bereitstellung von Forward Secrecy, die konkreten Implementierungsdetails liegen jedoch bei den Clients.

### Spezifikationen der AI Company Subnetzschicht

#### 6.1 Schlüsselzerstörungsspezifikationen

##### Zerstörung von Schlüsseln im Arbeitsspeicher
- **Nullüberschreibung**: Verwendung von memset_s oder einer äquivalenten Funktion zum Überschreiben des Schlüssel-Arbeitsspeichers
- **Mehrfaches Überschreiben**: Mindestens 3-mal mit verschiedenen Mustern überschreiben
- **Arbeitsspeichersperrung**: Verwendung von mlock/VirtualLock zur Verhinderung des Austauschs von Schlüsseln auf die Festplatte

##### Zerstörung persistenter Schlüssel
- **Sicheres Löschen**: Verwendung von shred oder einem äquivalenten Tool
- **SSD-Behandlung**: Spezielle Behandlung für SSDs (TRIM, sicheres Löschen)
- **Validierung der Zerstörung**: Bestätigung, dass Schlüsselmaterial nicht mehr wiederhergestellt werden kann

#### 6.2 Forward Secrecy-Garantie
- **Kurzlebigkeit von Sitzungsschlüsseln**: Schlüssel bei jeder Nachricht rotieren (Double Ratchet)
- **Keine Schlüsselzwischenspeicherung**: Keine Zwischenspeicherung von Schlüsseln, die zum Entschlüsseln alter Nachrichten verwendet werden könnten
- **Perfect Forward Secrecy (PFS)**: Explizite Garantie der PFS-Eigenschaft

---

## 7. Sicherheitsgrenzen und Tiefenverteidigung (AI Company Subnetzschicht)

### 7.1 Definition von Sicherheitsgrenzen

| Grenze | Schutzmaßnahmen |
|--------|-----------------|
| **Client-Grenze** | Sandbox-Isolation, Prozessisolation, Speicherschutz |
| **Netzwerkgrenze** | TLS 1.3, Certificate Pinning, Certificate Transparency |
| **Serviceknoten-Grenze** | Minimale Rechte, Netzwerkisolation, Intrusion Detection |
| **Datengrenze** | Ende-zu-Ende-Verschlüsselung, statische Verschlüsselung, Zugriffssteuerung |

### 7.2 Tiefenverteidigungsstrategie

```
Erste Ebene: Client-Sicherheit
    ↓
Zweite Ebene: Übertragungssicherheit (TLS 1.3 + Noise)
    ↓
Dritte Ebene: Ende-zu-Ende-Verschlüsselung (X3DH + Double Ratchet / MLS)
    ↓
Vierte Ebene: Mehrknoten-Validierung
    ↓
Fünfte Ebene: Audit-Protokolle und Anomalieerkennung (optional)
```

---

## 8. Sicherheits-Compliance und Zertifizierung (AI Company Subnetzschicht)

### 8.1 Compliance-Standards (bedarfsgerecht erfüllen)
- **GDPR**: Datenschutz, Benutzerkontrolle, Portabilität
- **HIPAA** (sofern zutreffend): Schutz medizinischer Daten
- **SOC 2**: Sicherheit, Verfügbarkeit, Vertraulichkeit
- **ISO 27001**: Informationssicherheits-Managementsystem

### 8.2 Sicherheitszertifizierung (optional)
- **Drittanbieter-Audit**: Regelmäßige Durchführung unabhängiger Sicherheitsaudits
- **Bug Bounty**: Einrichtung eines Bug Bounty-Programms
- **Öffentliche Transparenz**: Veröffentlichung von Sicherheitsauditberichten

---

## Zusammenfassung

### Überprüfung der schichtweisen Verantwortung

| Schicht | Verantwortung |
|---------|---------------|
| **Skynet-Protokollschicht** | Bereitstellung einer leichten, flexiblen Basis: kryptografische Primitive, Netzwerkkommunikation, Subnetzabstraktion |
| **AI Company Subnetzschicht** | Unternehmensweite Sicherheitsimplementierung: API-Key-Verwaltung, Audit-Protokolle, Compliance-Zertifizierung usw. |

### Klar definierte Verantwortlichkeiten

| Sicherheitsbereich | Skynet-Protokoll | AI Company |
|---------------------|------------------|------------|
| **Metadatenschutz** | Nicht bereitgestellt | Verantwortlich für die Implementierung |
| **Quantenresistente Kryptographie** | Erweiterungspunkte reserviert | Verantwortlich für die Implementierung |
| **API-Key-Verwaltung** | Nicht betroffen | Verantwortlich |
| **Audit-Protokolle** | Nicht enthalten | Verantwortlich |
| **Sybil-Schutz** | Nicht zwingend | Verantwortlich |

### Ergänzung des AI Company-Sicherheitsmodells

Als unternehmensweite Subnetz-Implementierung auf Skynet ergänzt das AI Company-Sicherheitsmodell die Protokollschicht auf folgende Weise:

1. **Metadatenschutz**: AI Company ist verantwortlich für die Implementierung und stellt Basismaßnahmen und langfristige Planungen bereit
2. **Quantenresistente Migration**: AI Company ist verantwortlich für die Implementierung, Hybrid-Zertifikatmodus, schrittweise Migrationsroadmap
3. **Client-Sicherheit**: Schlüsselverwaltungsspezifikationen, API-Key-Verwaltungsspezifikationen, Zufallszahlengenerierungsspezifikationen, offizielles SDK
4. **Audit-Protokolle**: AI Company ist verantwortlich für die Implementierung, vollständige Audit-Protokollfunktion
5. **Sybil-Schutz**: AI Company ist verantwortlich für die Implementierung, mehrschichtiger Schutz, Identitätsauthentifizierung, Reputationssystem
6. **Forward Secrecy**: Schlüsselzerstörungsspezifikationen, PFS-Garantie
7. **Compliance-Zertifizierung**: GDPR, SOC 2, ISO 27001

Zusammen bilden sie ein vollständiges unternehmensweites Sicherheitsystem, während gleichzeitig die Leichtgewichtigkeit und Flexibilität des Skynet-Protokolls selbst erhalten bleiben.
