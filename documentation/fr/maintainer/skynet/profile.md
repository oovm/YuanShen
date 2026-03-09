# Méta-informations utilisateur (Profile)

Les méta-informations utilisateur décrivent les informations publiques de l'utilisateur, y compris l'avatar, le pseudo, la bio personnelle, l'état en ligne, etc.

## Attributs des méta-informations

| Attribut | Description | Type |
|-----|------|------|
| **user_id** | ID utilisateur | user_id |
| **subnet_id** | ID de sous-réseau | subnet_id |
| **avatar** | Avatar | Référence de ressource ou URL (optionnel) |
| **nickname** | Pseudo | Nom d'affichage dans le sous-réseau (optionnel) |
| **bio** | Bio personnelle | Chaîne (optionnel) |
| **status_text** | Texte d'état personnalisé | Chaîne (optionnel, comme "En réunion", "Sorti pour manger") |
| **presence_status** | État en ligne | Énumération (online/busy/away/offline, optionnel) |
| **last_active_at** | Dernière heure d'activité | Horodatage (optionnel) |
| **updated_at** | Heure de mise à jour des méta-informations | Horodatage (optionnel) |
| **device_info** | Informations sur l'appareil | Objet JSON (optionnel, comme type d'appareil, système d'exploitation, etc.) |

## Types d'états en ligne

| État | Description | Icône |
|-----|------|------|
| **online** | L'utilisateur est actuellement actif et utilise le client | 🟢 |
| **busy** | L'utilisateur est en ligne mais occupé, peut ne pas répondre en temps opportun | 🔴 |
| **away** | L'utilisateur est en ligne mais absent depuis un certain temps (comme 5 minutes sans activité) | 🟡 |
| **offline** | L'utilisateur n'est pas en ligne ou n'a pas de connexion active | ⚪ |

## Mécanisme de mise à jour de l'état en ligne

### Mise à jour déclenchée par le client

- **Connexion** : Lorsque le client se connecte à un nœud de service, il est automatiquement défini sur l'état "online"
- **Activité** : Lorsque l'utilisateur effectue une action (envoyer un message, consulter un canal, etc.), met à jour la dernière heure d'activité
- **Absence** : Lorsque le client détecte que l'utilisateur n'a pas effectué d'action depuis un certain temps (comme 5 minutes), il est automatiquement défini sur l'état "away"
- **Occupé** : L'utilisateur peut définir manuellement l'état sur "busy"
- **Déconnexion** : Lorsque le client se déconnecte, il est automatiquement défini sur l'état "offline"

### Gestion par le nœud de service

- Le nœud de service maintient l'état en ligne de l'utilisateur
- Détecte si l'utilisateur est en ligne via un mécanisme de heartbeat
- Lorsque l'état change, diffuse aux utilisateurs concernés (comme les membres du même canal)

## Visibilité des méta-informations

- Les méta-informations utilisateur sont visibles publiquement dans le sous-réseau
- Peut configurer qui peut voir les méta-informations :
  - Tout le monde
  - Seulement les contacts
  - Seulement les membres du même canal
  - Personnalisé
- Les champs sensibles (comme device_info) peuvent avoir une visibilité configurée séparément

## Opérations sur les méta-informations

- **Voir les méta-informations** : Voir les méta-informations d'autres utilisateurs
- **Mettre à jour les méta-informations** : Mettre à jour son propre avatar, pseudo, bio personnelle
- **Définir l'état en ligne** : Définir manuellement son propre état en ligne (online/busy/away)
- **Définir le texte d'état** : Définir un texte d'état personnalisé
- **Effacer l'état** : Effacer l'état personnalisé, utiliser l'état par défaut

## Mécanisme de notification

- Lorsque les méta-informations utilisateur changent, peut notifier les utilisateurs concernés de manière sélective
- Prise en charge de l'activation/désactivation des notifications de changement d'état
- Peut configurer le suivi des changements de méta-informations d'utilisateurs spécifiques

## Considérations sur la vie privée

- Les méta-informations utilisateur sont publiques, mais ne contiennent pas d'informations sensibles
- Peut choisir de masquer l'état en ligne
- Les informations sur l'appareil sont optionnelles, l'utilisateur peut choisir de ne pas les partager
- Prise en charge du masquage partiel des méta-informations (comme afficher seulement le pseudo, pas l'avatar)
