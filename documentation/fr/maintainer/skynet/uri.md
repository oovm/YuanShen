# Protocole URI Skynet

Le protocole Skynet utilise `skynet://` comme protocole d'identificateur uniforme de ressource (URI), utilisé pour référencer divers types de ressources dans le réseau Skynet.

## Format du protocole

### Structure de base

```
skynet://[subnet_id]/[resource_type]/[resource_id][?query][#fragment]
```

### Description des composants

| Composant | Description | Requis |
|-----|------|------|
| **skynet://** | En-tête de protocole | Oui |
| **subnet_id** | ID de sous-réseau | Oui |
| **resource_type** | Type de ressource | Non |
| **resource_id** | ID de ressource | Non |
| **query** | Paramètres de requête | Non |
| **fragment** | Identificateur de fragment | Non |

## Types de ressources

### Types de ressources pris en charge

| Type de ressource | Description | Format de chemin |
|---------|------|---------|
| **subnet** | Sous-réseau | `skynet://{subnet_id}` |
| **user** | Utilisateur | `skynet://{subnet_id}/user/{user_id}` |
| **channel** | Canal | `skynet://{subnet_id}/channel/{channel_id}` |
| **message** | Message | `skynet://{subnet_id}/message/{message_id}` |
| **resource** | Ressource (fichier, etc.) | `skynet://{subnet_id}/resource/{resource_id}` |
| **thread** | Fil de discussion | `skynet://{subnet_id}/thread/{thread_id}` |

## Exemples

### 1. Référencer un sous-réseau

```
skynet://5f4dcc3b5aa765d61d8327deb882cf99
```

### 2. Référencer un utilisateur

```
skynet://5f4dcc3b5aa765d61d8327deb882cf99/user/user_123
```

### 3. Référencer un canal

```
skynet://5f4dcc3b5aa765d61d8327deb882cf99/channel/channel_456
```

### 4. Référencer un message

```
skynet://5f4dcc3b5aa765d61d8327deb882cf99/message/msg_789
```

### 5. Référencer une ressource

```
skynet://5f4dcc3b5aa765d61d8327deb882cf99/resource/res_abc
```

### 6. Référencer un fil

```
skynet://5f4dcc3b5aa765d61d8327deb882cf99/thread/msg_789
```

### 7. Avec paramètres de requête

```
skynet://5f4dcc3b5aa765d61d8327deb882cf99/message/msg_789?highlight=true&scroll_to_bottom=true
```

## Spécifications d'encodage

- **subnet_id** : Utiliser la représentation hexadécimale (minuscule) de la valeur de hachage Blake3
- **user_id**, **channel_id**, **message_id**, **resource_id**, **thread_id** : Utiliser un encodage Base64 sécurisé pour les URL ou hexadécimal
- **Caractères spéciaux** : Les caractères spéciaux dans les composants de chemin doivent être encodés en URL

## Principe d'isolation des sous-réseaux

L'URI Skynet suit le principe d'isolation des sous-réseaux :
- Toutes les URI doivent contenir `subnet_id`
- Les ressources ne sont pas partagées entre les sous-réseaux, une copie doit être faite si une utilisation entre sous-réseaux est nécessaire
- Les ressources dans l'URI ne sont accessibles que dans le sous-réseau spécifié

## Traitement côté client

Le client doit, lors de l'analyse d'une URI `skynet://` :
1. Vérifier le format de l'URI
2. Vérifier si le sous-réseau cible a été rejoint
3. Exécuter l'action correspondante en fonction du type de ressource (comme rediriger, ouvrir la ressource, etc.)
4. Pour les sous-réseaux non rejoints, peut fournir un guide d'adhésion
