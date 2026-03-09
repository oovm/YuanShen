# Calendriers/Tâches planifiées

Les calendriers/tâches planifiées sont un mécanisme d'exécution de tâches piloté par le temps dans AI Company, utilisé pour exécuter automatiquement des flux de travail ou des tâches à des moments spécifiés ou selon des cycles fixes.

## Concept central

### Calendrier (Schedule)

Un calendrier est un conteneur pour les tâches pilotées par le temps, définissant les règles temporelles d'exécution des tâches et le contenu à exécuter.

### Types de déclenchement

Les calendriers prennent en charge plusieurs types de déclenchement :

- **Déclenchement unique** : Exécuter une fois à un moment spécifique indiqué
- **Déclenchement périodique** : Exécuter répétitivement selon un cycle fixe (comme quotidiennement, hebdomadairement, mensuellement, etc.)
- **Déclenchement conditionnel** : Déclenché en combinaison avec le temps et d'autres conditions

## Structure du calendrier

Un calendrier contient les attributs clés suivants :

| Attribut | Description |
|---------|-------------|
| **Nom** | Nom identifiant du calendrier |
| **Description** | Description détaillée du calendrier |
| **Règle de déclenchement** | Expression temporelle définissant quand le calendrier se déclenche |
| **Contenu d'exécution** | Flux de travail ou tâche à exécuter lors du déclenchement du calendrier |
| **Projet associé** | Projet auquel appartient le calendrier (optionnel) |
| **État** | État de fonctionnement du calendrier (activé/désactivé) |

## Règles de déclenchement

Les calendriers utilisent des expressions cron standard ou une configuration temporelle simplifiée pour définir les règles de déclenchement :

- **Expression Cron** : Expression temporelle puissante et flexible, prenant en charge les secondes, minutes, heures, jours, mois, semaines
- **Configuration simplifiée** :
  - Heure spécifique quotidienne
  - Jour et heure spécifiques hebdomadaires
  - Date et heure spécifiques mensuelles
  - Intervalle personnalisé (comme toutes les 2 heures)

## Contenu d'exécution

Les calendriers peuvent déclencher les types de contenu d'exécution suivants :

- **Flux de travail** : Exécuter un flux de travail complet
- **Tâche unique** : Exécuter une tâche unique spécifique
- **Notification de rappel** : Envoyer une notification de rappel de calendrier

## Scénarios d'utilisation

Les calendriers/tâches planifiées conviennent aux scénarios suivants :

- **Rapports périodiques** : Générer automatiquement des rapports de progression de projet quotidiennement/hebdomadairement
- **Synchronisation des données** : Synchroniser périodiquement les données des systèmes externes
- **Tâches de sauvegarde** : Exécuter périodiquement des sauvegardes de données
- **Notifications de rappel** : Envoyer des rappels à des moments importants
- **Traitement par lots** : Exécuter le traitement par lots de données pendant les heures creuses

## Relation avec d'autres concepts

- **Projet** : Les calendriers peuvent être associés à des projets spécifiques et s'exécuter dans le contexte du projet
- **Flux de travail** : Les calendriers sont généralement utilisés pour déclencher l'exécution de flux de travail
- **Tâche** : Les calendriers peuvent également déclencher directement l'exécution de tâches uniques

## Fonctionnalités de gestion

Les calendriers fournissent les fonctionnalités de gestion suivantes :

- **Activation/Désactivation** : Contrôler à tout moment si le calendrier s'exécute
- **Déclenchement manuel** : Prendre en charge l'exécution immédiate manuelle du calendrier
- **Historique d'exécution** : Enregistrer chaque situation et résultat d'exécution du calendrier
- **Réessai en cas d'échec** : Mécanisme de réessai automatique en cas d'échec de l'exécution
