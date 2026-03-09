# Protocole Skynet

Skynet est le protocole de communication décentralisé d'AI Company, conçu sur la base d'une architecture à confiance zéro, réalisant un réseau de messagerie instantanée sécurisé et protégeant la vie privée.

## Portée du protocole

Le protocole Skynet **est responsable** de :
- Authentification d'identité de nœud et découverte de réseau
- Communication chiffrée de bout en bout (X3DH + Double Ratchet pour les conversations privées, MLS pour les conversations de groupe)
- Stockage et synchronisation des données de sous-réseau (réplication tolérante aux pannes byzantines)
- Définition des structures de données pour les messages, les ressources et les méta-informations utilisateur

Le protocole Skynet **n'est pas responsable** de :
- Gestion des clés (stockage, sauvegarde, récupération) - à la charge du client
- Génération de nombres aléatoires et sources d'entropie - à la charge du client
- Cryptographie résistante aux quantiques - hors sujet, pas abordé pour le moment
- Mécanismes de sécurité client comme les modules de sécurité matérielle, la biométrie, etc. - à la charge du client

Voir [Limites du protocole et clause de non-responsabilité](./skynet.md#10-limites-du-protocole-et-clause-de-non-responsabilit%C3%A9).

## Concepts clés

### 1. Identifiants clés
Le protocole Skynet comporte trois identifiants clés :
- **auth_id** : ID d'identité globale de l'utilisateur, unique entre les sous-réseaux, similaire à un vrai nom. **Secret le plus élevé du protocole Skynet, utilisé uniquement pour l'authentification d'identité, ne doit jamais être divulgué ou stocké dans les données internes du sous-réseau.**
- **user_id** : ID local de l'utilisateur dans le sous-réseau, similaire à un pseudo/pseudo, utilisé pour la protection de la vie privée. **Toutes les données internes du sous-réseau n'utilisent que user_id.**
- **subnet_id** : ID unique du sous-réseau, identifiant un système de communication indépendant

### 2. Modèle de sous-réseau
Skynet adopte un modèle de sous-réseau, chaque système de communication ou communauté indépendant est un sous-réseau :
- Chaque sous-réseau a ses propres utilisateurs, contacts, groupes
- Les utilisateurs peuvent rejoindre plusieurs sous-réseaux en même temps, en utilisant le même `auth_id`
- Chaque sous-réseau a des contacts et des groupes indépendants (les contacts sont un concept interne au sous-réseau)

### 3. Architecture décentralisée
Skynet adopte une structure de réseau à deux niveaux :
- **Réseau principal** : Réseau pair-à-pair composé de nœuds de service, responsable de la découverte des nœuds et du routage réseau
- **Sous-réseau** : Système de communication indépendant, isolant les données des différentes plateformes

### 4. Nœud de service
- Chaque nœud possède une paire de clés Ed25519 à long terme et un NodeID unique
- Les nœuds effectuent la découverte de réseau et le stockage d'adresses via Kademlia DHT
- Prise en charge des protocoles STUN/TURN pour le traversée NAT

### 5. Sous-réseau
Le sous-réseau est un système de communication indépendant :
- Chaque sous-réseau a un subnet_id unique
- Un ensemble de nœuds de maintenance est responsable du stockage et de la synchronisation des données
- Prise en charge de la gestion des membres, des canaux (channel) et d'autres fonctions internes au sous-réseau

### 6. Ressource (Resource)
La ressource est une entité accessible dans le sous-réseau :
- La ressource a un ID de ressource unique (resource_id)
- La ressource peut être un fichier, une image, un document, un lien, etc.
- La ressource peut être accédée et gérée avec autorisation
- Prise en charge du contrôle de version et de l'historique des ressources
- **Pas de partage entre sous-réseaux** : La ressource n'est accessible que dans le sous-réseau auquel elle appartient, une copie doit être faite si une utilisation entre sous-réseaux est nécessaire

### 7. Message (Message)
Le message est une unité de contenu transmise dans le sous-réseau :
- Le message a un ID de message unique (message_id)
- Le contenu du message est **des données structurées** (objet JSON), pas un format Markdown ou HTML fixe
- Le client décide autonomement comment le rendre en fonction du type de message et de la structure du contenu
- Le message prend en charge le chiffrement de bout en bout
- Le message peut être envoyé dans un canal ou une conversation privée
- **Pas de transmission entre sous-réseaux** : Les messages ne traversent pas les sous-réseaux, une copie doit être faite dans le sous-réseau cible si un transfert est nécessaire
- **État de message optionnel** : Prise en charge du suivi de l'état (livré, lu, etc.) (extension optionnelle)

### 8. Réaction emoji au message (Reaction)
La réaction emoji au message est le retour émotionnel de l'utilisateur sur un message :
- L'utilisateur peut ajouter une réaction emoji au message (comme 👍, ❤️, 🎉, etc.)
- Prise en charge des emojis personnalisés
- Peut voir quels utilisateurs ont ajouté quel emoji
- Peut annuler sa propre réaction

### 9. Épinglage de message (Pin)
L'épinglage de message est utilisé pour marquer les messages importants :
- Peut épingler des messages importants en haut du canal ou de la conversation privée
- Prise en charge de l'épinglage de plusieurs messages
- Les messages épinglés sont triés par temps ou manuellement
- Seuls les administrateurs ou l'expéditeur du message peuvent épingler (configurable)

### 10. Fil de discussion (Thread)
Le fil de discussion est utilisé pour des discussions approfondies sous un message :
- Peut créer un fil de réponse sous n'importe quel message
- Les messages du fil sont affichés indépendamment, sans affecter le flux de chat principal
- Prise en charge des notifications de message dans le fil
- Peut réduire/développer le fil

### 11. Mention de message (Mention)
La mention de message est utilisée pour alerter un utilisateur spécifique :
- **@utilisateur** : Mentionne un utilisateur spécifique, cet utilisateur recevra une notification
- **@tout le monde** : Mentionne tous les membres du canal
- **@rôle** : Mentionne tous les membres d'un rôle spécifique (comme @administrateur)
- Les messages mentionnés sont mis en évidence

### 12. Méta-informations utilisateur (Profile)
Les méta-informations utilisateur décrivent les informations publiques de l'utilisateur :
- Avatar (avatar) : Image ou hachage de l'avatar de l'utilisateur
- Pseudo (nickname) : Nom d'affichage de l'utilisateur dans le sous-réseau
- Bio (bio) : Bio personnelle de l'utilisateur
- Texte d'état personnalisé (status_text) : Texte d'état personnalisé de l'utilisateur
- État de présence (presence_status) : État en ligne de l'utilisateur (en ligne/occupé/absent/hors ligne)
- Dernière heure d'activité (last_active_at) : Heure de la dernière activité de l'utilisateur
- Les méta-informations sont publiques, stockées dans la liste des membres du sous-réseau

### 13. Sécurité à confiance zéro
- Toutes les opérations critiques nécessitent une vérification de signature cryptographique
- Aucune confiance entre les nœuds de service, entre le client et le serveur
- Le chiffrement de bout en bout garantit que seules les parties communicantes peuvent déchiffrer le contenu

## Navigation dans la documentation

| Document | Description |
|------|------|
| [Projet de conception du protocole Skynet](./skynet.md) | Documentation technique complète du protocole Skynet, y compris l'identité de nœud, l'authentification utilisateur, le chiffrement de bout en bout, la synchronisation de stockage de données, etc. |
| [Modèle de menace](./threat-model.md) | Analyse des menaces de sécurité auxquelles est confronté le protocole Skynet à l'aide du cadre STRIDE et des mesures d'atténuation |
| [Protocole URI](./uri.md) | Spécification du protocole URI Skynet, définissant le format et l'utilisation de l'en-tête de protocole skynet:// |
| [Modèle de sous-réseau](./subnets.md) | Détails d'implémentation spécifiques du sous-réseau, y compris l'identification du sous-réseau, la structure de données, les nœuds de maintenance, etc. |
| [Modèle de ressource](./resources.md) | Détails d'implémentation spécifiques de la ressource, y compris le type de ressource, le stockage, les autorisations, etc. |
| [Modèle de message](./messages.md) | Détails d'implémentation spécifiques du message, y compris le type de message, le format, l'état, la réaction, l'épinglage, le fil, la mention, etc. |
| [Méta-informations utilisateur](./profile.md) | Détails d'implémentation spécifiques des méta-informations utilisateur, y compris l'avatar, le pseudo, l'état en ligne, etc. |

## Relation avec la conception architecturale

Le protocole Skynet est l'implémentation technique concrète de la **philosophie de conception décentralisée** dans la [conception architecturale](../architecture/index.md) d'AI Company.

### Diagramme de relation

```
Philosophie de conception décentralisée (architecture/)
    ↓ Guide
Implémentation du protocole Skynet (skynet/)
    ↓ Implémente
Fonctionnement réel du système
```

### Correspondance spécifique

| Philosophie de conception | Implémentation Skynet |
|---------|-----------|
| Architecture à confiance zéro | Authentification d'identité auth_id, chiffrement de bout en bout, hypothèse de non-confiance mutuelle |
| Décentralisation | Réseau pair-à-pair, structure à deux niveaux réseau principal-sous-réseau, pas de point de défaillance unique |
| Calcul vérifiable | Signatures numériques, vérification croisée multi-nœuds, tolérance aux pannes byzantines |
| Souveraineté des données | Chiffrement de bout en bout, contrôle des clés par l'utilisateur, le serveur ne peut pas accéder au texte en clair |

Voir la [Conception architecturale](../architecture/index.md) pour comprendre la philosophie de conception.
