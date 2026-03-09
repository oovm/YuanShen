# Nœuds de travail

## Concept central

### Nœud de travail (Worker Node)

Un **nœud de travail** est l'unité de calcul de base dans le système AI Company. **Chaque appareil physique ou virtuel est un nœud de travail indépendant**.

Les nœuds de travail fournissent un environnement d'exécution pour les espaces de travail. Un espace de travail peut s'exécuter simultanément sur plusieurs nœuds de travail, avec une instance de cet espace de travail sur chaque nœud.

#### Types de nœuds de travail

Le système prend en charge les types de nœuds de travail suivants :

| Type de nœud | Description | Scénarios typiques | Caractéristiques |
|--------------|-------------|-------------------|-----------------|
| **PC Windows personnel** | Appareil de bureau | Travail de bureau quotidien, création de contenu | Haute performance, grand écran, périphériques d'entrée complets |
| **Serveur** | Serveur cloud ou local | Tâches en arrière-plan, services à long terme | Fonctionnement 24h/24, haute fiabilité, traitement multitâche |
| **Téléphone Android** | Appareil mobile Android | Travail mobile, réponse immédiate | Portabilité, accès à tout moment, capteurs riches |
| **Téléphone iOS** | Appareil mobile Apple | Travail mobile, réponse immédiate | Portabilité, haute sécurité, écosystème complet |
| **Tablette** | Tablette Android/iPad | Consommation de contenu, création légère | Grand écran, convivialité tactile |
| **Appareil IoT** | Matériel intelligent | Tâches spécifiques, collecte de données | Fonction dédiée, faible consommation |

#### Caractéristiques des nœuds de travail

Chaque nœud de travail possède les caractéristiques suivantes :

1. **Identité indépendante**
   - Chaque nœud a un ID de nœud unique
   - Prend en charge l'authentification et l'autorisation des nœuds
   - Peut rejoindre ou quitter le système indépendamment

2. **Local en priorité**
   - Utiliser en priorité les ressources de calcul locales
   - Peut toujours fonctionner normalement hors ligne
   - Synchronisation des données effectuée automatiquement en arrière-plan

3. **Découverte des ressources**
   - Découverte et connexion automatiques des nœuds
   - Ajustement dynamique de la topologie des nœuds
   - Prend en charge le branchement à chaud des nœuds

4. **Déclaration des capacités**
   - Les nœuds déclarent leurs propres capacités (comme GPU, logiciels spécifiques, capteurs)
   - Les tâches sont routées vers les nœuds appropriés
   - Mise à jour dynamique des capacités

## Conception architecturale

### Structure hiérarchique Nœud-Espace de travail

```
Système AI Company
│
├── Espace de travail A (Espace de travail personnel, sur 3 nœuds)
│   ├── Nœud de travail 1 (PC Windows)
│   │   └── Instance d'espace de travail A-1
│   │       ├── Agent 1
│   │       └── Cache de données local
│   ├── Nœud de travail 2 (Téléphone Android)
│   │   └── Instance d'espace de travail A-2
│   │       ├── Agent 2 (migré depuis le nœud 1)
│   │       └── Cache de données local
│   └── Nœud de travail 3 (Serveur)
│       └── Instance d'espace de travail A-3
│           ├── Agent 3 (exécuté en arrière-plan)
│           └── Copie complète des données
│
├── Espace de travail B (Espace de travail de projet, sur 2 nœuds)
│   ├── Nœud de travail 1 (PC Windows)
│   │   └── Instance d'espace de travail B-1
│   │       └── Agent 4
│   └── Nœud de travail 4 (Serveur)
│       └── Instance d'espace de travail B-2
│           └── Agent 5 (exécuté à long terme)
│
└── Espace de travail C (Espace de travail temporaire, nœud unique)
    └── Nœud de travail 2 (Téléphone Android)
        └── Instance d'espace de travail C-1
            └── Tâche temporaire
```

### Communication entre nœuds

Les nœuds de travail prennent en charge la découverte automatique, la communication sécurisée et la synchronisation des données. Pour des choix technologiques détaillés, veuillez vous référer aux [Choix technologiques](../../maintainer/technology-choices.md) dans la documentation des mainteneurs.

## Gestion des nœuds de travail

### Enregistrement des nœuds

1. **Premier démarrage**
   - Le nœud génère une paire de clés d'identité unique
   - Crée un fichier de configuration de nœud
   - S'enregistre dans le système

2. **Informations du nœud**
   ```
   Informations du nœud {
     id: Identifiant unique du nœud
     name: Nom du nœud
     type: Windows | Server | Android | iOS
     capabilities: [Liste des capacités]
     status: online | offline | busy
     lastSeen: Dernière heure en ligne
     workspaces: [Liste des espaces de travail associés]
   }
   ```

### État des nœuds

Les nœuds de travail ont plusieurs états, indiquant leur disponibilité et leur charge. Pour des informations détaillées, veuillez vous référer à la documentation des mainteneurs.

### Surveillance des nœuds

- **Surveillance des ressources** : CPU, mémoire, disque, réseau
- **Vérification de l'état de santé** : Détection régulière par heartbeat
- **Indicateurs de performance** : Taux d'achèvement des tâches, temps de réponse
- **Mécanisme d'alerte** : Notification en cas d'anomalie

## Gestion des instances d'espace de travail

Chaque espace de travail a une instance sur les nœuds associés :

| État de l'instance | Description |
|-------------------|-------------|
| **Active** | Instance active, agent en cours d'exécution |
| **Standby** | Instance en attente, synchronisation des données en cours |
| **Offline** | Nœud hors ligne, données à synchroniser |
| **Failed** | Instance anormale, nécessite une récupération |

### Cycle de vie de l'instance

1. **Création** : Créer une instance lors de l'allocation de l'espace de travail à un nœud
2. **Démarrage** : Initialiser l'instance, charger les données
3. **Exécution** : Exécuter les agents, traiter les tâches
4. **Synchronisation** : Synchroniser les données avec d'autres instances
5. **Arrêt** : Suspendre les agents, sauvegarder l'état
6. **Destruction** : Supprimer l'instance du nœud

## Ordonnancement des agents

### Sélection des nœuds

Le système sélectionne le nœud de travail approprié (dans la plage de l'espace de travail) en fonction des facteurs suivants :

1. **Correspondance des capacités** : Le nœud dispose-t-il des capacités requises
2. **Disponibilité des ressources** : Les ressources du nœud sont-elles suffisantes
3. **Latence réseau** : Prioriser les nœuds avec une latence faible
4. **Emplacement des données** : Prioriser le nœud où se trouvent les données
5. **Préférences utilisateur** : Respecter les préférences prioritaires de l'utilisateur
6. **État de l'instance** : Prioriser les instances en état Active

### Migration des tâches

- **Migration dynamique** : Les agents peuvent migrer entre différents nœuds de l'espace de travail
- **Conservation de l'état** : Conserver l'état et le contexte de l'agent lors de la migration
- **Basculement transparent** : L'utilisateur ne perçoit pas le processus de migration

## Gestion des données

### Hiérarchie de stockage des données

1. **Niveau instance** : Cache local de l'instance d'espace de travail
2. **Niveau espace de travail** : Données partagées de l'espace de travail (synchronisées entre les nœuds)
3. **Niveau global** : Données partagées entre les espaces de travail

### Stratégie de synchronisation des données

- **Synchronisation en temps réel** : Synchronisation en temps réel des données importantes
- **Synchronisation périodique** : Synchronisation périodique des données non critiques
- **Synchronisation à la demande** : Synchronisation déclenchée manuellement par l'utilisateur
- **Résolution de conflits** : Résolution automatique ou manuelle des conflits

## Modèle de sécurité

### Sécurité des nœuds

- **Authentification des nœuds** : Chaque nœud a un certificat d'identité indépendant
- **Démarrage sécurisé** : Vérification de sécurité lors du démarrage du nœud
- **Effacement à distance** : Effacement à distance des données pour les appareils perdus
- **Autorisation des espaces de travail** : Contrôler quels espaces de travail un nœud peut rejoindre

## Relation avec les flux de travail

Les nœuds de travail fournissent des ressources de calcul pour les flux de travail, et les tâches des flux de travail peuvent être distribuées à différents nœuds pour exécution. Sélection automatique en fonction des capacités et des ressources des nœuds, prenant en charge la migration des tâches entre les nœuds.

Voir la documentation des [Flux de travail](./workflows.md) pour en savoir plus.

## Résumé

Les concepts de nœud de travail et d'espace de travail fournissent une architecture de calcul distribuée flexible, efficace et sécurisée pour AI Company :

- **Flexibilité** : Les espaces de travail peuvent s'étendre sur plusieurs nœuds, prenant en charge divers types d'appareils, s'adaptant à différents scénarios
- **Efficacité** : Ordonnancement intelligent, utilisation optimisée des ressources, migration automatique des agents
- **Sécurité** : Isolation multicouche, contrôle d'autorisation à granularité fine, autorisation des nœuds
- **Expérience utilisateur** : Basculement transparent, utilisable hors ligne, synchronisation des données

Cette conception permet aux utilisateurs de vraiment posséder et contrôler leurs propres ressources de calcul, tout en profitant des avantages d'un système distribué. L'espace de travail n'est plus limité à un seul appareil, mais à une entité logique qui peut traverser les limites des nœuds.
