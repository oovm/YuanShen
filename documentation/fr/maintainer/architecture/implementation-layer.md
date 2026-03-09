# Couche d'implémentation (Modules centraux augur-*)

Ce document décrit en détail les modules centraux de la couche d'implémentation dans le système AI Company. La couche d'implémentation se situe entre la couche de présentation et la couche de protocole, fournissant la logique métier centrale et l'implémentation des fonctionnalités.

## Aperçu

La couche d'implémentation est composée de plusieurs modules `augur-*` indépendants, chacun responsable d'un domaine fonctionnel spécifique. Ces modules collaborent via l'injection de dépendances et une conception d'interface claire, offrant une base fonctionnelle solide aux applications de niveau supérieur.

## Liste des modules

La couche d'implémentation contient les modules centraux suivants :

| Nom du module | Responsabilité | État |
|---------------|----------------|------|
| [augur-agent](#augur-agent) | Gestion des agents | Core |
| [augur-orchestrator](#augur-orchestrator) | Orchestration des tâches et gestion des workflows | Core |
| [augur-organization](#augur-organization) | Gestion des organisations, départements et rôles | Core |
| [augur-skill](#augur-skill) | Plugins de compétences et extension des capacités | Core |
| [augur-memory](#augur-memory) | Gestion du système de mémoire | Core |
| [augur-persistence](#augur-persistence) | Implémentation de la couche de persistance | Infrastructure |
| [augur-file-system](#augur-file-system) | Service de stockage de fichiers | Infrastructure |
| [augur-types](#augur-types) | Définitions de types et gestion des erreurs | Infrastructure |

## Diagramme de dépendances des modules

```
┌─────────────────────────────────────────────────────────────────┐
│                      Couche Logique Métier                        │
├─────────────────────────────────────────────────────────────────┤
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐          │
│  │augur-agent   │  │augur-orchestr│  │augur-organiz │          │
│  │(Gestion Agent)│  │ator(Orchestre)│  │ation(Organis.)│          │
│  └──────────────┘  └──────────────┘  └──────────────┘          │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐          │
│  │augur-skill   │  │augur-memory  │  │              │          │
│  │(Gestion Skill)│  │(Système Mémoire)│  │              │          │
│  └──────────────┘  └──────────────┘  └──────────────┘          │
└─────────────────────────────────────────────────────────────────┘
                            ↓ Dépendance
┌─────────────────────────────────────────────────────────────────┐
│                     Couche Infrastructure                         │
├─────────────────────────────────────────────────────────────────┤
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐          │
│  │augur-persist │  │augur-file-sys│  │augur-types   │          │
│  │ence(Persistance)│  │tem(Système Fichier)│  │(Définitions Types)│          │
│  └──────────────┘  └──────────────┘  └──────────────┘          │
└─────────────────────────────────────────────────────────────────┘
                            ↓ Utilisation
┌─────────────────────────────────────────────────────────────────┐
│                   Couche Protocole (Skynet)                        │
└─────────────────────────────────────────────────────────────────┘
```

## Description détaillée des modules

### augur-types

**Responsabilité :** Fournir les définitions de types partagés et un mécanisme unifié de gestion des erreurs pour la couche d'implémentation.

**Fonctionnalités principales :**
- Définition des structures de données centrales (espace de travail, projet, entreprise, équipe, employé, compétence, etc.)
- Type d'erreur unifié `AugurError` et mécanisme de gestion des erreurs
- Fourniture de types enum standard (tels que `ProjectStatus`, `RoleType`, `SkillLevel`, etc.)
- Prise en charge des messages d'erreur internationalisés (via clé i18n)

**Structures principales :**
- `Workspace` - Espace de travail
- `Project` - Projet
- `Company` - Entreprise
- `Team` - Équipe
- `Employee` - Employé
- `Skill` - Compétence

**Relations de dépendance :** Aucune dépendance aux autres modules augur-*, est la base de tous les autres modules.

### augur-persistence

**Responsabilité :** Fournir une implémentation unifiée de la couche de persistance, prenant en charge plusieurs backends de bases de données.

**Fonctionnalités principales :**
- Prise en charge de plusieurs bases de données (SQLite, PostgreSQL)
- Conception d'interface Repository unifiée
- Gestion des migrations de base de données
- Prise en charge de la persistance des types d'entités

**Types d'entités pris en charge :**
- Utilisateur
- Organisation
- Département
- Rôle
- Conversation
- Message
- Agent
- Mémoire
- Étiquette de mémoire

**Exemple d'utilisation :**
```rust
use augur_persistence::{AugurPersistence, PersistenceConfig, DatabaseType};

let config = PersistenceConfig::default();
let persistence = AugurPersistence::new(config).await?;

let user_repo = persistence.user_repository();
```

**Relations de dépendance :** Dépend de `augur-types`, est dépendé par les modules de logique métier.

### augur-file-system

**Responsabilité :** Fournir un service de stockage de fichiers basé sur le système de fichiers local.

**Fonctionnalités principales :**
- Téléchargement et téléversement de fichiers
- Gestion des métadonnées de fichiers
- Contrôle du niveau d'accès aux fichiers
- Recherche et filtrage de fichiers
- Listes de fichiers par utilisateur et organisation
- Opérations de copie et déplacement de fichiers
- Prise en charge de la détection de plusieurs types MIME

**Exemple d'utilisation :**
```rust
use augur_file_system::{FsFileService, FsFileServiceConfig};

let config = FsFileServiceConfig::default();
let service = FsFileService::new(config).await?;
```

**Relations de dépendance :** Dépend de `augur-types`, est dépendé par les modules de logique métier.

### augur-memory

**Responsabilité :** Implémenter le système de mémoire, prenant en charge la création, le stockage, la récupération et la gestion des mémoires.

**Fonctionnalités principales :**
- Opérations CRUD sur les mémoires
- Gestion des étiquettes de mémoire
- Gestion des relations entre mémoires
- Recherche par mots-clés
- Filtrage par étiquettes et plage de temps
- Importation et exportation de mémoires
- Extraction de contexte de conversation
- Création de mémoires à partir de conversations

**Exemple d'utilisation :**
```rust
use augur_memory::AugurMemory;
use std::sync::Arc;

let memory_service = AugurMemory::new(
    Arc::new(memory_repository),
    Arc::new(memory_tag_repository),
);
```

**Relations de dépendance :** Dépend de `augur-types` et `augur-persistence`, est dépendé par `augur-agent`, etc.

### augur-agent

**Responsabilité :** Fournir les définitions de types et interfaces liés aux agents, gérer le cycle de vie des agents.

**Fonctionnalités principales :**
- Définition des types d'agents
- Gestion de l'état des agents
- Interface de configuration des agents
- Gestion du cycle de vie des agents

**Relations de dépendance :** Dépend de `augur-types`, `augur-memory` et `augur-skill`, est dépendé par les applications de niveau supérieur.

### augur-orchestrator

**Responsabilité :** Fournir les types et interfaces pour l'orchestration des tâches et la gestion des workflows.

**Fonctionnalités principales :**
- Interface d'orchestration des tâches
- Gestion des workflows
- Planification des tâches
- Gestion des dépendances

**Relations de dépendance :** Dépend de `augur-types`, est dépendé par les applications de niveau supérieur.

### augur-organization

**Responsabilité :** Fournir les types et interfaces pour la gestion des organisations, départements et rôles.

**Fonctionnalités principales :**
- Gestion des organisations
- Structure des départements
- Rôles et permissions
- Gestion des membres

**Relations de dépendance :** Dépend de `augur-types` et `augur-persistence`, est dépendé par les applications de niveau supérieur.

### augur-skill

**Responsabilité :** Fournir les types et interfaces pour les plugins de compétences et l'extension des capacités.

**Fonctionnalités principales :**
- Interface de plugin de compétences
- Définition des compétences
- Exécution des compétences
- Gestion des plugins

**Relations de dépendance :** Dépend de `augur-types`, est dépendé par `augur-agent`.

## Exemples de collaboration entre modules

### Exemple 1 : Exécution d'une tâche par un agent

1. L'application de niveau supérieur appelle `augur-orchestrator` pour initier une tâche
2. `augur-orchestrator` recherche un agent `augur-agent` disponible
3. `augur-agent` utilise `augur-skill` pour charger les compétences nécessaires
4. Pendant l'exécution, `augur-agent` utilise `augur-memory` pour enregistrer et récupérer des mémoires
5. Tous les changements d'état sont persistés via `augur-persistence`

### Exemple 2 : Collaboration organisationnelle

1. L'application de niveau supérieur gère la structure organisationnelle via `augur-organization`
2. `augur-organization` utilise `augur-persistence` pour stocker les données organisationnelles
3. Les fichiers téléchargés par les utilisateurs sont gérés via `augur-file-system`
4. Les agents au sein de l'organisation collaborent via `augur-agent`

## Principes de conception

La couche d'implémentation suit les principes de conception suivants :

1. **Principe de responsabilité unique** - Chaque module n'est responsable que d'un domaine fonctionnel clairement défini
2. **Principe d'inversion des dépendances** - Les modules de niveau supérieur ne dépendent pas des modules de niveau inférieur, tous dépendent d'abstractions
3. **Principe de ségrégation des interfaces** - Fournir des interfaces minimisées, éviter les interfaces volumineuses
4. **Conception modulaire** - Les modules collaborent via des limites claires et des relations de dépendance
5. **Testabilité** - Chaque module doit être facilement testable de manière indépendante

## Guide d'extension

Lorsque vous avez besoin d'ajouter de nouvelles fonctionnalités, suivez les étapes suivantes :

1. Déterminez à quel module existant la fonctionnalité appartient, ou si un nouveau module doit être créé
2. Définissez les structures de données et types d'erreur nécessaires dans `augur-types`
3. Implémentez la logique centrale dans le module métier correspondant
4. Si la persistance est nécessaire, ajoutez un Repository dans `augur-persistence`
5. Si le stockage de fichiers est nécessaire, utilisez `augur-file-system`
6. Mettez à jour ce document pour enregistrer la description des nouvelles fonctionnalités

## Documents connexes

- [Aperçu de l'architecture](./index.md)
- [Conception décentralisée](./decentralization.md)
- [Modèle de sécurité](./security-model.md)
- [Modèles de données](../data-models.md)
- [Protocole Skynet](../skynet/index.md)
