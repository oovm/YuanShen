# Modèle de message

Le message est une unité de contenu transmise dans le sous-réseau, utilisée pour la communication entre les utilisateurs.

## Attributs de message

| Attribut | Description | Type |
|-----|------|------|
| **message_id** | ID unique du message | Chaîne |
| **channel_id** | ID du canal | ID du canal (conversation de groupe) ou vide (conversation privée) |
| **sender_id** | ID de l'expéditeur | user_id |
| **recipient_id** | ID du destinataire | user_id (utilisé pour les conversations privées) |
| **message_type** | Type de message | Chaîne (comme text/image/file/resource, etc., type ouvert) |
| **content** | Contenu du message | Données structurées (objet JSON ou données structurées chiffrées) |
| **content_encrypted** | Si chiffré | Booléen |
| **resource_refs** | Liste des références de ressources | Tableau de resource_id (optionnel) |
| **reply_to** | ID du message auquel on répond | message_id (optionnel) |
| **thread_id** | ID du fil | message_id (optionnel, utilisé pour les fils de discussion) |
| **mentions** | Liste des mentions | Tableau d'objets de mention (optionnel) |
| **is_pinned** | Si épinglé | Booléen (optionnel, par défaut false) |
| **pinned_at** | Heure d'épinglage | Horodatage (optionnel) |
| **pinned_by** | ID de l'épingleur | user_id (optionnel) |
| **status** | État du message | Chaîne (optionnel, comme sending/sent/delivered/read/recalled) |
| **delivered_at** | Heure de livraison | Horodatage (optionnel) |
| **read_at** | Heure de lecture | Horodatage (optionnel) |
| **created_at** | Heure de création | Horodatage |
| **edited_at** | Heure de modification | Horodatage (optionnel) |
| **deleted_at** | Heure de suppression | Horodatage (optionnel, suppression douce) |
| **metadata** | Métadonnées personnalisées | Objet JSON (optionnel) |

## Types de messages

Le type de message est un identifiant de chaîne ouvert, exemples courants :

| Type de message | Description |
|---------|------|
| **text** | Message texte |
| **image** | Message image |
| **file** | Message fichier |
| **resource** | Message de référence de ressource |
| **link** | Message lien |
| **system** | Message système |

## Format du contenu du message

- Le contenu du message est **des données structurées** (généralement un objet JSON)
- Pas un format Markdown ou HTML fixe
- Le client décide autonomement comment le rendre en fonction de `message_type` et de la structure de `content`
- Le contenu texte peut contenir des balises de formatage comme Markdown, HTML, etc., mais c'est au client de décider de les analyser et de les rendre

### Exemple de structure de contenu par type de message

- **text** : Message texte
  - content contient le champ `text` (texte brut, Markdown, HTML, etc., le client décide de la manière de rendre)
- **image** : Message image
  - content contient le champ `resource_id` référençant la ressource image
  - Peut contenir optionnellement le champ `caption` (description de l'image)
- **file** : Message fichier
  - content contient le champ `resource_id` référençant la ressource fichier
  - Peut contenir optionnellement des métadonnées comme `filename`, `size`, etc.
- **resource** : Message de référence de ressource
  - content contient le champ `resource_id`
- **link** : Message lien
  - content contient le champ `url`
  - Peut contenir optionnellement des informations d'aperçu comme `title`, `description`, `preview_image`, etc.
- **system** : Message système (comme membre rejoint, canal créé, etc.)
  - content contient des champs comme `event_type`, `event_data`, etc.

## Opérations sur les messages

- **Envoi de message** : Envoyer un message à un canal ou une conversation privée, chiffrement de bout en bout (optionnel)
- **Réception de message** : Récupérer de nouveaux messages depuis les nœuds de maintenance
- **Modification de message** : Modifier un message déjà envoyé, conserver l'historique des modifications
- **Suppression de message** : Marquer le message comme supprimé (suppression douce)
- **Réponse à un message** : Répondre à un message spécifique
- **Historique de messages** : Récupérer l'historique des messages d'un canal ou d'une conversation privée

## Sécurité des messages

- Prise en charge du chiffrement de bout en bout, seules les parties communicantes peuvent déchiffrer
- Hachage du contenu du message pour la vérification de l'intégrité
- Signature de l'expéditeur pour l'authentification d'identité
- Les nœuds de service ne peuvent pas accéder au contenu du message chiffré

## Principe d'isolation des sous-réseaux

- Les messages **ne traversent pas** les sous-réseaux, chaque sous-réseau est un domaine de communication indépendant
- Si un transfert de message entre sous-réseaux est nécessaire, il doit être **copié** dans le sous-réseau cible, plutôt que référencé directement
- Après la copie, la copie dans le sous-réseau cible a un cycle de vie indépendant

## Gestion de l'état des messages (extension optionnelle)

Le sous-réseau peut choisir d'activer la fonction de suivi de l'état des messages, offrant une expérience de message plus riche :

| État | Description |
|-----|------|
| **Envoi en cours** | Le message a été soumis au nœud de maintenance, en cours de traitement |
| **Envoyé** | Le message a été stocké sur le nœud de maintenance, en attente de réception |
| **Livré** | Le message a été poussé vers l'appareil en ligne du destinataire |
| **Lu** | Le destinataire a lu ce message |
| **Rappelé** | L'expéditeur a rappelé ce message |

### Opérations d'état de message (optionnelles)

- **Mise à jour de l'état** : Lorsque l'appareil du destinataire est en ligne, mettre à jour automatiquement à "Livré"
- **Accusé de réception de lecture** : Après que le destinataire a lu le message, envoyer une notification de lecture (optionnel)
- **Rappel de message** : L'expéditeur peut rappeler le message dans un certain délai (optionnel)
- **Visibilité de l'état** : Peut configurer qui peut voir l'état du message (seulement l'expéditeur/seulement les deux parties/tout le monde)

## Réaction emoji au message (Reaction)

La réaction emoji au message est le retour émotionnel de l'utilisateur sur un message.

### Attributs de réaction

| Attribut | Description | Type |
|-----|------|------|
| **reaction_id** | ID unique de la réaction | Chaîne |
| **message_id** | ID du message cible | message_id |
| **user_id** | Utilisateur ayant ajouté la réaction | user_id |
| **emoji** | Emoji | Chaîne (comme "👍", "❤️", "🎉") ou ID d'emoji personnalisé |
| **created_at** | Heure d'ajout | Horodatage |

### Opérations de réaction

- **Ajouter une réaction** : L'utilisateur ajoute une réaction emoji à un message
- **Annuler une réaction** : L'utilisateur annule sa propre réaction
- **Requête de réactions** : Récupérer la liste de toutes les réactions d'un message
- **Voir les utilisateurs** : Voir la liste des utilisateurs ayant ajouté un emoji spécifique

### Caractéristiques des réactions

- Prise en charge des emojis Unicode standard
- Prise en charge des emojis personnalisés (doivent être définis au niveau du sous-réseau)
- Le même utilisateur ne peut ajouter qu'une fois le même emoji au même message
- Le même utilisateur peut ajouter plusieurs emojis différents au même message

## Épinglage de message (Pin)

L'épinglage de message est utilisé pour marquer les messages importants, les affichant en haut du canal ou de la conversation privée.

### Attributs d'épinglage

| Attribut | Description | Type |
|-----|------|------|
| **pin_id** | ID unique de l'épinglage | Chaîne |
| **message_id** | ID du message épinglé | message_id |
| **channel_id** | ID du canal | ID du canal (conversation de groupe) ou vide (conversation privée) |
| **pinned_by** | ID de l'épingleur | user_id |
| **pinned_at** | Heure d'épinglage | Horodatage |
| **order** | Position de tri | Entier (optionnel, utilisé pour le tri manuel) |

### Opérations d'épinglage

- **Épingler un message** : Épingler un message en haut du canal ou de la conversation privée
- **Désépingler un message** : Annuler l'état d'épinglage d'un message
- **Récupérer la liste des épinglés** : Récupérer tous les messages épinglés d'un canal ou d'une conversation privée
- **Ajuster l'ordre** : Ajuster l'ordre d'affichage des messages épinglés

### Autorisations d'épinglage

- Peut configurer qui a le droit d'épingler des messages :
  - Seulement les administrateurs
  - Seulement l'expéditeur du message
  - Tous les membres
  - Rôle personnalisé

## Fil de discussion (Thread)

Le fil de discussion est utilisé pour des discussions approfondies sous un message spécifique, sans affecter le flux de chat principal.

### Attributs de fil

| Attribut | Description | Type |
|-----|------|------|
| **thread_id** | ID du fil | message_id (c'est-à-dire le message_id du message parent) |
| **parent_message_id** | ID du message parent | message_id |
| **channel_id** | ID du canal | ID du canal (conversation de groupe) ou vide (conversation privée) |
| **created_by** | Créateur du fil | user_id |
| **created_at** | Heure de création du fil | Horodatage |
| **message_count** | Nombre de messages dans le fil | Entier |
| **last_message_at** | Heure du dernier message | Horodatage |

### Messages du fil

Les messages dans le fil sont similaires aux messages ordinaires, mais avec les différences suivantes :
- Le champ `thread_id` du message du fil est défini sur le `message_id` du message parent
- Les messages du fil sont affichés indépendamment dans l'interface utilisateur
- Peut réduire/développer le fil

### Opérations de fil

- **Créer un fil** : Créer un fil de réponse sous un message
- **Envoyer un message de fil** : Envoyer un message dans le fil
- **Voir le fil** : Voir tous les messages dans le fil
- **Réduire/développer le fil** : Contrôler l'état d'affichage du fil
- **Notifications de fil** : Peut configurer si recevoir des notifications pour les messages dans le fil

## Mention de message (Mention)

La mention de message est utilisée pour alerter des utilisateurs ou des groupes spécifiques.

### Types d'objets de mention

| Type | Description | Exemple |
|-----|------|------|
| **user** | Mentionner un utilisateur spécifique | @alice |
| **channel** | Mentionner tous les membres du canal | @tout le monde, @channel |
| **role** | Mentionner un rôle spécifique | @administrateur, @groupe de développement |

### Attributs de mention

| Attribut | Description | Type |
|-----|------|------|
| **mention_type** | Type de mention | user/channel/role |
| **mention_id** | ID de l'objet mentionné | user_id ou role_id |
| **mention_name** | Nom d'affichage de la mention | Chaîne |
| **offset** | Position de départ dans le texte | Entier (optionnel, utilisé pour la mise en évidence du texte) |
| **length** | Longueur dans le texte | Entier (optionnel, utilisé pour la mise en évidence du texte) |

### Opérations de mention

- **Ajouter une mention** : Ajouter une mention dans un message
- **Analyser une mention** : Le client analyse les mentions dans le message
- **Envoyer une notification** : Les utilisateurs mentionnés reçoivent une notification
- **Mise en évidence** : La partie mentionnée est mise en évidence dans le message

### Exemple de structure de contenu de mention

Pour les messages de type text, content peut contenir :
```json
{
  "text": "Bonjour à tous, @alice veuillez consulter ce document",
  "mentions": [
    {
      "mention_type": "user",
      "mention_id": "user_123",
      "mention_name": "alice",
      "offset": 5,
      "length": 6
    }
  ]
}
```
