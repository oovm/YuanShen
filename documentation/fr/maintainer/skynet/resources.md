# Modèle de ressource

La ressource est une entité accessible dans le sous-réseau, stockée et gérée par les nœuds de maintenance du sous-réseau.

## Attributs de ressource

| Attribut | Description | Type |
|-----|------|------|
| **resource_id** | ID unique de la ressource | Chaîne |
| **resource_type** | Type de ressource | Chaîne (comme file/image/document/link, etc., type ouvert) |
| **name** | Nom de la ressource | Chaîne |
| **description** | Description de la ressource | Chaîne (optionnel) |
| **content_hash** | Hachage du contenu de la ressource | Hachage blake3 (utilisé pour vérifier l'intégrité) |
| **content_uri** | Emplacement de stockage du contenu de la ressource | URI (optionnel, comme CID IPFS, stockage local, etc.) |
| **size** | Taille de la ressource | Entier (octets) |
| **created_at** | Heure de création | Horodatage |
| **created_by** | Créateur | user_id |
| **updated_at** | Heure de mise à jour | Horodatage (optionnel) |
| **updated_by** | Metteur à jour | user_id (optionnel) |
| **version** | Numéro de version | Entier |
| **permissions** | Paramètres d'autorisation | Objet JSON (optionnel) |
| **metadata** | Métadonnées personnalisées | Objet JSON (optionnel) |

## Types de ressources

Le type de ressource est un identifiant de chaîne ouvert, exemples courants :

| Type de ressource | Description |
|---------|------|
| **file** | Fichier générique |
| **image** | Fichier image |
| **document** | Fichier document |
| **video** | Fichier vidéo |
| **audio** | Fichier audio |
| **link** | Ressource de lien |

## Opérations sur les ressources

- **Téléchargement de ressource** : Créer une nouvelle ressource, calculer content_hash, stocker le contenu
- **Téléchargement de ressource** : Obtenir la ressource en fonction de resource_id, vérifier content_hash
- **Mise à jour de ressource** : Mettre à jour le contenu de la ressource, augmenter le numéro de version, conserver les versions historiques
- **Suppression de ressource** : Marquer la ressource comme supprimée (suppression douce)
- **Requête de ressource** : Rechercher et filtrer la liste des ressources

## Autorisations de ressource

- Le créateur de la ressource peut définir les autorisations d'accès
- Prise en charge de niveaux d'autorisation comme lecture seule, lecture/écriture, gestion, etc.
- Peut autoriser des utilisateurs, des rôles ou un canal entier spécifiques

## Principe d'isolation des sous-réseaux

- Les ressources **ne sont pas partagées** entre les sous-réseaux, les ressources ne sont accessibles que dans le sous-réseau auquel elles appartiennent
- Si une utilisation entre sous-réseaux de la ressource est nécessaire, elle doit être **copiée** dans le sous-réseau cible, plutôt que référencée directement
- Après la copie, la copie dans le sous-réseau cible a un cycle de vie et des autorisations indépendants
