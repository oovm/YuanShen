# Espaces de travail

## Concept central

### Espace de travail (Workspace)

Un **espace de travail** est une unité logique dans le système AI Company pour isoler et organiser différentes tâches, projets ou environnements. **Un espace de travail peut exister et s'exécuter sur plusieurs nœuds de travail**, réalisant une véritable collaboration distribuée.

L'espace de travail n'est plus limité à un seul nœud, mais à une entité logique qui peut traverser les limites des appareils.

#### Rôle de l'espace de travail

1. **Isolation de l'environnement**
   - Les données et configurations de différents espaces de travail sont isolées les unes des autres
   - Éviter les interférences entre les tâches
   - Prend en charge des stratégies de sécurité indépendantes

2. **Collaboration entre nœuds**
   - L'espace de travail peut s'exécuter simultanément sur plusieurs nœuds
   - Les agents peuvent migrer et collaborer entre différents nœuds
   - Les données se synchronisent automatiquement entre plusieurs nœuds

3. **Allocation des ressources**
   - Allouer des ressources spécifiques à chaque espace de travail (peut s'étendre sur plusieurs nœuds)
   - Limiter l'utilisation des ressources, empêcher l'épuisement des ressources
   - Ordonnancement par priorité

4. **Gestion du contexte**
   - Sauvegarder l'état du contexte de l'espace de travail
   - Basculement rapide entre différents scénarios de travail
   - Prend en charge la pause et la reprise des espaces de travail

#### Types d'espaces de travail

| Type d'espace de travail | Utilisation | Caractéristiques |
|-------------------------|-------------|-----------------|
| **Espace de travail personnel** | Utilisation quotidienne personnelle | Entièrement privé, configuration flexible |
| **Espace de travail de projet** | Collaboration sur un projet spécifique | Partageable, contrôle de version |
| **Espace de travail d'agent** | Exécution d'agents AI | Gestion automatique, optimisation des ressources |
| **Espace de travail sécurisé** | Traitement de tâches sensibles | Niveau de sécurité le plus élevé, suivi d'audit |
| **Espace de travail temporaire** | Tâches à court terme | Nettoyage automatique, cycle de vie court |

#### Relation Espace de travail - Nœud

```
Espace de travail (multi-nœuds)
│
├── Nœud de travail 1 (PC Windows)
│   ├── Instance d'espace de travail A
│   │   ├── Agent 1
│   │   └── Cache de données local
│   └── Instance d'espace de travail B
│       └── ...
│
├── Nœud de travail 2 (Téléphone Android)
│   └── Instance d'espace de travail A
│       ├── Agent 2 (migré depuis le nœud 1)
│       └── Cache de données local
│
└── Nœud de travail 3 (Serveur)
    └── Instance d'espace de travail A
        ├── Agent 3 (exécuté en arrière-plan)
        └── Copie complète des données
```

## Gestion des espaces de travail

### Création d'un espace de travail

1. **Méthodes de création**
   - Création manuelle : L'utilisateur crée activement un espace de travail
   - Création automatique : Création automatique en fonction des besoins de la tâche
   - Création à partir de modèle : Création basée sur un modèle prédéfini

2. **Configuration de l'espace de travail**
   ```
   Configuration de l'espace de travail {
     name: Nom de l'espace de travail
     type: personal | project | agent | secure | temp
     nodeIds: [Liste des nœuds associés]  // Peut s'étendre sur plusieurs nœuds
     resources: {
       cpu: Limite CPU (somme entre les nœuds)
       memory: Limite mémoire (somme entre les nœuds)
       storage: Limite de stockage (somme entre les nœuds)
     }
     security: {
       encryption: Chiffrer ou non
       accessControl: Contrôle d'accès
     }
     lifecycle: {
       autoCleanup: Nettoyer automatiquement ou non
       ttl: Durée de vie
     }
     distribution: {
       preferredNodes: [Nœuds préférés]
       failoverNodes: [Nœuds de basculement]
       replicationStrategy: full | partial | selective
     }
   }
   ```

### Stratégie de distribution des espaces de travail

1. **Mode maître-esclave**
   - Un nœud maître responsable de la coordination
   - Les autres nœuds en tant que répliques
   - Convient aux scénarios de séparation lecture/écriture

2. **Mode pair à pair**
   - Tous les nœuds sont égaux en statut
   - Les données sont répliquées entre les nœuds
   - Convient aux scénarios de haute disponibilité

3. **Mode partitionnement**
   - Les données de l'espace de travail sont partitionnées sur différents nœuds
   - Chaque nœud est responsable d'une partie des données
   - Convient aux scénarios de données à grande échelle

### Basculement des espaces de travail

1. **Basculement rapide**
   - Basculement d'espace de travail en un clic
   - Sauvegarder le contexte actuel
   - Restaurer l'état de l'espace de travail cible

2. **Espaces de travail parallèles**
   - Prend en charge l'exécution simultanée de plusieurs espaces de travail
   - Isolation des ressources entre les espaces de travail
   - Peut configurer la priorité des espaces de travail

### Synchronisation des espaces de travail

1. **Synchronisation entre nœuds**
   - Les données de l'espace de travail se synchronisent automatiquement entre plusieurs nœuds
   - Synchronisation incrémentielle, réduire la bande passante
   - Stratégie de résolution de conflits

2. **Synchronisation sélective**
   - Peut sélectionner les nœuds à synchroniser
   - Peut configurer la direction de synchronisation
   - Prend en charge l'édition hors ligne

3. **Migration des agents**
   - Les agents peuvent migrer entre différents nœuds de l'espace de travail
   - Conserver l'état et le contexte de l'agent
   - Ordonnancement automatique en fonction de la situation des ressources

## Gestion des instances de nœud

Chaque espace de travail a une instance sur les nœuds associés :

| État de l'instance | Description |
|-------------------|-------------|
| **Active** | Instance active, agent en cours d'exécution |
| **Standby** | Instance en attente, synchronisation des données en cours |
| **Offline** | Nœud hors ligne, données à synchroniser |
| **Failed** | Instance anormale, nécessite une récupération |

## Modèle de sécurité

### Sécurité de l'espace de travail

- **Chiffrement de l'espace de travail** : Stockage chiffré des données de l'espace de travail
- **Contrôle d'accès** : Contrôle d'autorisation d'accès à granularité fine
- **Journal d'audit** : Enregistrement complet de l'audit des opérations
- **Autorisation des nœuds** : Contrôler quels nœuds peuvent rejoindre l'espace de travail

## Relation avec les flux de travail

L'espace de travail fournit un environnement d'exécution pour les flux de travail, et les flux de travail s'exécutent à l'intérieur de l'espace de travail. Les flux de travail peuvent s'exécuter sur plusieurs instances de nœuds de l'espace de travail, réalisant un flux de tâches distribué.

Voir la documentation des [Flux de travail](./workflows.md) pour en savoir plus.
