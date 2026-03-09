# Modèle de sous-réseau

## Identifiant de sous-réseau

Chaque sous-réseau possède un ID unique `subnet_id = blake3(octets arbitraires)`, qui peut être généré par le créateur. Le réseau principal est un sous-réseau prédéfini, avec un ID fixe composé de tous les zéros (`0x00...`).

## Type de sous-réseau

Le type de sous-réseau est un identifiant de chaîne ouvert, utilisé pour distinguer la fonction et l'usage du sous-réseau. Le client peut décider de charger et de traiter le sous-réseau en fonction de son type.

**Exemples courants de types de sous-réseaux** :

| Type de sous-réseau | Description | Exemple d'usage |
|---------|------|---------|
| **organization** | Sous-réseau d'organisation | Pour la communication et la collaboration d'organisations et d'entreprises |
| **community** | Sous-réseau de communauté | Pour la communication de communautés ordinaires et de groupes d'intérêt |
| **private** | Sous-réseau privé | Pour la communication de petits groupes privés |

**Rôle du type de sous-réseau** :
- Le client peut décider de charger et de traiter le sous-réseau en fonction de son type
- Différents types de sous-réseaux peuvent avoir différentes fonctionnalités et stratégies d'autorisation
- Le client peut charger et afficher les sous-réseaux de manière sélective en fonction du type
- Le type de sous-réseau est ouvert, les applications peuvent définir leurs propres types

## Méta-informations du sous-réseau

Chaque sous-réseau contient des méta-informations, utilisées pour décrire ses attributs de base et ses fonctionnalités :

| Attribut | Description | Type |
|-----|------|------|
| **subnet_type** | Type de sous-réseau | Chaîne (type ouvert, comme organization/community/private) |
| **subnet_version** | Version du type de sous-réseau | Chaîne (comme "1.0.0") |
| **name** | Nom du sous-réseau | Chaîne |
| **description** | Description du sous-réseau | Chaîne (optionnel) |
| **icon** | Icône du sous-réseau | URL ou hachage (optionnel) |
| **created_at** | Heure de création | Horodatage |
| **created_by** | Créateur | user_id |
| **features** | Liste des fonctionnalités | Tableau de chaînes (optionnel) |
| **metadata** | Métadonnées personnalisées | Objet JSON (optionnel) |

**Rôle des méta-informations du sous-réseau** :
- Le client détermine s'il prend en charge ce sous-réseau via `subnet_type`
- `subnet_type_version` est utilisé pour la vérification de compatibilité de version
- La liste `features` déclare les fonctionnalités prises en charge par le sous-réseau
- `metadata` peut être utilisé pour stocker des informations d'extension spécifiques au type

## Concept de modèle de sous-réseau

Skynet adopte un modèle de sous-réseau, chaque système de communication ou communauté indépendant est un sous-réseau :
- Chaque sous-réseau a ses propres utilisateurs, contacts, groupes
- Les utilisateurs peuvent rejoindre plusieurs sous-réseaux en même temps, en utilisant le même `auth_id`
- Chaque sous-réseau a des contacts et des groupes indépendants (les contacts sont un concept interne au sous-réseau)

## Identifiants clés du sous-réseau

En tant que protocole de communication décentralisé, Skynet a trois identifiants clés pour les sous-réseaux :

| Identifiant | Description | Rôle |
|-----|------|------|
| **auth_id** | ID d'identité globale de l'utilisateur | Identifie l'utilisateur de manière unique entre les sous-réseaux, similaire à un vrai nom. **Secret le plus élevé du protocole Skynet, utilisé uniquement pour l'authentification d'identité, ne doit jamais être divulgué ou stocké dans les données internes du sous-réseau.** |
| **user_id** | ID local de l'utilisateur dans le sous-réseau | Identifiant local dans le sous-réseau, similaire à un pseudo/pseudo, utilisé pour la protection de la vie privée. **Toutes les données internes du sous-réseau n'utilisent que user_id.** |
| **channel_id** | ID de canal/groupe | Identifie un groupe de chat ou un canal dans le sous-réseau |

## Données du sous-réseau

Chaque sous-réseau maintient les données suivantes et utilise le **stockage par réplication multi-nœuds** :

### 1. Méta-informations du sous-réseau

- **subnet_type** : Type de sous-réseau (chaîne ouverte, comme organization/community/private)
- **subnet_type_version** : Version du type de sous-réseau
- **name** : Nom du sous-réseau
- **description** : Description du sous-réseau (optionnel)
- **icon** : Icône du sous-réseau (optionnel)
- **created_at** : Heure de création
- **created_by** : user_id du créateur
- **features** : Liste des fonctionnalités (optionnel)
- **metadata** : Métadonnées personnalisées (optionnel)

### 2. Liste des membres
- **user_id** : ID local de l'utilisateur dans le sous-réseau (unique dans le sous-réseau, optionnel)
- **Clé publique d'identité** : Clé publique Ed25519 de l'utilisateur (utilisée pour vérifier les signatures)
- **Rôle** : Administrateur / Membre ordinaire
- **Heure d'adhésion** : Heure à laquelle l'utilisateur a rejoint le sous-réseau
- **Liste des clés publiques des appareils** : Clés publiques des appareils utilisés par l'utilisateur dans ce sous-réseau (utilisées pour le chiffrement de bout en bout)
- **Liste des contacts** : Contacts de l'utilisateur dans le sous-réseau (concept interne au sous-réseau, n'utilise que user_id)
- **Méta-informations utilisateur (Profile)** :
  - **avatar** : Avatar (référence de ressource ou URL, optionnel)
  - **nickname** : Pseudo (nom d'affichage dans le sous-réseau, optionnel)
  - **bio** : Bio personnelle (optionnel)
  - **status_text** : Texte d'état personnalisé (comme "En réunion", "Sorti pour manger", optionnel)
  - **presence_status** : État en ligne (online/busy/away/offline, optionnel)
  - **last_active_at** : Dernière heure d'activité (horodatage, optionnel)
  - **updated_at** : Heure de mise à jour des méta-informations (horodatage, optionnel)
  - **device_info** : Informations sur l'appareil (objet JSON, optionnel, comme type d'appareil, système d'exploitation, etc.)

**Remarque** : `auth_id` n'est utilisé que pour l'authentification d'identité initiale, ne doit jamais être stocké dans la liste des membres du sous-réseau ou dans aucune donnée interne du sous-réseau.

### 2.1 Mode de sous-réseau : Ouvert vs Fermé
Le sous-réseau prend en charge deux modes de fonctionnement :
| Mode | Description | Scénarios d'application |
|------|------|---------|
| **Sous-réseau fermé** | L'adhésion nécessite une vérification par l'administrateur, prévention des attaques Sybil | Entreprises, organisations, communautés privées |
| **Sous-réseau ouvert** | Tout le monde peut rejoindre librement, sans vérification | Communautés publiques, groupes d'intérêt |

Le mode est spécifié lors de la création du sous-réseau et peut être modifié ultérieurement par l'administrateur.

### 3. Liste des canaux
- **channel_id** : ID unique du canal/groupe
- **Nom du canal** : Nom du groupe
- **Créateur** : user_id du créateur
- **Heure de création** : Heure de création
- **Liste des membres** : Liste des user_id des membres du canal
- **Type de canal** : Conversation privée / Conversation de groupe / Canal d'annonce

### 4. Stratégie d'autorisation
- Qui peut ajouter/supprimer des membres
- Qui peut créer/supprimer des canaux
- Qui peut modifier les informations du canal

### 5. Ressource (Resource)

La ressource est une entité accessible dans le sous-réseau, stockée et gérée par les nœuds de maintenance du sous-réseau. Voir [Modèle de ressource](./resources.md).

### 6. Message (Message)

Le message est une unité de contenu transmise dans le sous-réseau, utilisée pour la communication entre les utilisateurs. Voir [Modèle de message](./messages.md).

## Nœuds de maintenance du sous-réseau

Chaque sous-réseau est responsable du stockage et de la synchronisation des données par un ensemble de nœuds de service (appelés **nœuds de maintenance**). Lors de la création du sous-réseau, le créateur peut spécifier les nœuds de maintenance initiaux (généralement les nœuds en qui il a confiance), et ultérieurement ajouter/supprimer des nœuds de maintenance via un vote dans le sous-réseau ou une désignation par l'administrateur.

Les nœuds de maintenance maintiennent la cohérence des données via la **réplication tolérante aux pannes byzantines**.
