# Conception architecturale

Ce répertoire contient la documentation sur la philosophie et les principes de conception architecturale d'AI Company.

## Liste des documents

- [decentralization.md](decentralization.md) - Conception décentralisée : philosophie de conception de sécurité centrale du système
- [security-model.md](security-model.md) - Modèle de sécurité : guide d'implémentation de sécurité de couche d'architecture basé sur le protocole Skynet
- [presentation-layer.md](presentation-layer.md) - Couche de présentation : description détaillée des applications frontales et des services backends
- [implementation-layer.md](implementation-layer.md) - Couche d'implémentation : description détaillée des modules centraux augur-*
- [protocol-layer.md](protocol-layer.md) - Couche de protocole : description détaillée des modules du protocole Skynet

## Architecture en trois couches

AI Company adopte une conception claire en trois couches, chaque couche ayant des responsabilités et des limites claires.

### Diagramme de relation des couches d'architecture

```
┌─────────────────────────────────────────────────────────────┐
│                   Couche de présentation (côté application) │
├─────────────────────────────────────────────────────────────┤
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐    │
│  │  ai-company  │  │  ai-empire   │  │  ai-planet   │    │
│  │  (Application principale)     │  │  (Module Empire)   │  │  (Module Planète)   │    │
│  └──────────────┘  └──────────────┘  └──────────────┘    │
│  ┌──────────────┐  ┌──────────────┐                         │
│  │  ai-waifu    │  │  Application frontend    │                         │
│  │  (Module Personnage)   │  │  (Vue.js)     │                         │
│  └──────────────┘  └──────────────┘                         │
└─────────────────────────────────────────────────────────────┘
                            ↓ Appel
┌─────────────────────────────────────────────────────────────┐
│              Couche d'implémentation (modules centraux augur-*) │
├─────────────────────────────────────────────────────────────┤
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐    │
│  │augur-agent   │  │augur-orchestr│  │augur-organiz │    │
│  │(Gestion d'agents)   │  │ator(Orchestrateur)  │  │ation(Organisation)   │    │
│  └──────────────┘  └──────────────┘  └──────────────┘    │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐    │
│  │augur-skill   │  │augur-memory  │  │augur-persist │    │
│  │(Gestion de compétences)     │  │(Système de mémoire)     │  │ence(Persistance)  │    │
│  └──────────────┘  └──────────────┘  └──────────────┘    │
│  ┌──────────────┐  ┌──────────────┐                         │
│  │augur-file-sys│  │augur-types   │                         │
│  │tem(Système de fichiers)  │  │(Définitions de types)     │                         │
│  └──────────────┘  └──────────────┘                         │
└─────────────────────────────────────────────────────────────┘
                            ↓ Utilisation
┌─────────────────────────────────────────────────────────────┐
│                  Couche de protocole (Protocole Skynet)                        │
├─────────────────────────────────────────────────────────────┤
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐    │
│  │skynet-types  │  │skynet-auth   │  │skynet-chat   │    │
│  │(Types de base)     │  │(Protocole d'authentification)     │  │(Protocole de chat)     │    │
│  └──────────────┘  └──────────────┘  └──────────────┘    │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐    │
│  │skynet-gateway│  │skynet-memory │  │skynet-service│    │
│  │(Protocole de passerelle)     │  │(Protocole de stockage)     │  │(Protocole de service)     │    │
│  └──────────────┘  └──────────────┘  └──────────────┘    │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐    │
│  │skynet-bridge │  │skynet-notific│  │skynet-persist│    │
│  │(Protocole de pont)     │  │ation(Notification)    │  │ence(Persistance)  │    │
│  └──────────────┘  └──────────────┘  └──────────────┘    │
└─────────────────────────────────────────────────────────────┘
```

### Description des responsabilités de chaque couche

#### 1. Couche de présentation (côté application)

La couche de présentation est l'interface avec laquelle l'utilisateur interagit directement, y compris divers types d'applications et d'interfaces frontales.

**Composants principaux :**
- `ai-company` - Backend de l'application principale
- `ai-empire` - Backend du module Empire
- `ai-planet` - Backend du module Planète
- `ai-waifu` - Backend du module Personnage
- Application frontend Vue.js

**Responsabilités :**
- Fournir l'interface utilisateur
- Traiter les interactions utilisateur
- Appeler les modules centraux de la couche d'implémentation
- Afficher et collecter les données

#### 2. Couche d'implémentation (modules centraux augur-*)

La couche d'implémentation est la couche de logique métier centrale du système, contenant plusieurs modules fonctionnels spécialisés.

**Modules principaux :**
- `augur-agent` - Gestion d'agents
- `augur-orchestrator` - Orchestrateur
- `augur-organization` - Gestion d'organisation
- `augur-skill` - Gestion de compétences
- `augur-memory` - Système de mémoire
- `augur-persistence` - Persistance
- `augur-file-system` - Système de fichiers
- `augur-types` - Définitions de types

**Responsabilités :**
- Implémenter la logique métier centrale
- Coordonner la collaboration entre les modules
- Gérer les agents et les compétences
- Fournir des fonctionnalités de collaboration organisationnelle

#### 3. Couche de protocole (Protocole Skynet)

La couche de protocole fournit les protocoles de communication et d'échange de données sous-jacents, et est l'infrastructure de tout le système.

**Protocoles principaux :**
- `skynet-types` - Définitions de types de base
- `skynet-auth` - Protocole d'authentification
- `skynet-chat` - Protocole de chat
- `skynet-gateway` - Protocole de passerelle
- `skynet-memory` - Protocole de stockage
- `skynet-service` - Protocole de service
- `skynet-bridge` - Protocole de pont
- `skynet-notification` - Protocole de notification
- `skynet-persistence` - Protocole de persistance

**Responsabilités :**
- Fournir des protocoles de communication standardisés
- Gérer l'authentification et la sécurité
- Gérer le stockage et la transmission des données
- Prendre en charge la communication décentralisée

## Relation avec Skynet

Le protocole Skynet constitue la couche d'infrastructure (couche de protocole) du système AI Company. Ce répertoire décrit la **philosophie de conception et les principes architecturaux** d'AI Company, tandis que le [Protocole Skynet](../skynet/index.md) est l'**implémentation technique concrète** de ces principes de conception.

### Positionnement de Skynet dans l'architecture en trois couches

Dans l'architecture en trois couches, le protocole Skynet est l'infrastructure la plus basse :

```
Couche de présentation (côté application)
    ↓ Appel
Couche d'implémentation (modules centraux augur-*)
    ↓ Utilisation
Couche de protocole (Protocole Skynet) ← C'est ici que se trouve Skynet
```

### Diagramme de relation globale

```
Concepts centraux AI Company (concepts/)
    ↓ Construit sur
Implémentation d'architecture en trois couches (couche de présentation + couche d'implémentation)
    ↓ Basé sur
Protocole Skynet (couche de protocole)
    ↓ Suit
Philosophie de conception décentralisée (architecture/)
```

### Mappage entre les concepts AI Company et les concepts Skynet

Les concepts centraux d'AI Company sont construits sur le protocole Skynet :

| Concept AI Company | Concept correspondant Skynet | Description |
|----------------|-----------------|------|
| **Organisation** | **Sous-réseau (Subnet)** | Chaque organisation correspond à un sous-réseau indépendant |
| **Utilisateur (utilisateur humain réel)** | **Utilisateur de sous-réseau (user_id/auth_id)** | L'utilisateur réel correspond à une identité d'utilisateur dans le sous-réseau |
| **Carnet d'adresses - Contacts internes** | **user_id** | Les contacts internes ne nécessitent que user_id |
| **Carnet d'adresses - Contacts externes** | **subnet_id + user_id** | Les contacts externes nécessitent le couple subnet_id + user_id |
| **Structure organisationnelle** | **Spécifique à AI Company** | La structure organisationnelle est un concept construit par AI Company au-dessus du sous-réseau |
| **Équipe (Agent Cluster)** | **Spécifique à AI Company** | L'équipe est un concept d'unité de collaboration d'agents unique à AI Company |
| **Projet (Project)** | **Spécifique à AI Company** | Le projet est lié à un canal (non modifiable) et à une équipe (modifiable), avec le concept d'administrateur de projet |
| **Canal (Channel)** | **Canal (Channel)** | Le canal est un concept de groupe de chat natif Skynet, non modifiable après liaison avec un projet |
| **Tâche (Task)** | **Spécifique à AI Company** | La tâche est un concept d'unité de travail sous un projet |
| **Employé (AI Agent)** | **Spécifique à AI Company** | L'employé AI Agent est un concept de conteneur pour un ensemble de compétences |
| **Compétence (Skill)** | **Spécifique à AI Company** | La compétence est une unité de capacité de travail interne de l'Agent |
| **Capacité (Capability)** | **Spécifique à AI Company** | La capacité est une fonctionnalité d'extension externe de l'Agent (y compris API, MCP, etc.) |
| **Espace de travail (Workspace)** | **Spécifique à AI Company** | L'espace de travail est un concept de collaboration distribuée unique à AI Company |
| **Nœud de travail (Worker Node)** | **Nœud de service (Service Node)** | Le nœud de travail est le nœud de service Skynet |
| **Flux de travail (Workflow)** | **Spécifique à AI Company** | Le flux de travail est un concept d'orchestration de tâches unique à AI Company |
| **Calendrier/Tâche programmée (Schedule)** | **Spécifique à AI Company** | Le calendrier/tâche programmée est un concept de tâche pilotée par le temps unique à AI Company |
| **Rapport de travail (Report)** | **Spécifique à AI Company** | Le rapport de travail (rapport quotidien, hebdomadaire, mensuel) est un concept de résumé des résultats de travail des Agents AI, des équipes et des tâches |

### Mappage entre la philosophie de conception et l'implémentation technique

| Philosophie de conception | Implémentation Skynet |
|---------|-----------|
| Architecture à confiance zéro | Authentification d'identité de nœud, chiffrement de bout en bout |
| Décentralisation | Réseau pair-à-pair, structure à deux niveaux réseau principal-sous-réseau |
| Calcul vérifiable | Signatures numériques, vérification croisée multi-nœuds |
| Souveraineté des données | Chiffrement de bout en bout, contrôle des clés par l'utilisateur |

Voir le [Protocole Skynet](../skynet/index.md) pour l'implémentation technique concrète, et voir les [Concepts centraux](../../concepts/index.md) pour les concepts de niveau supérieur d'AI Company.
