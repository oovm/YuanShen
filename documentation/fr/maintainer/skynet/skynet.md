# Projet de Design du Protocole Skynet

## 1. Aperçu
Skynet est un protocole de communication décentralisé visant à construire un réseau de messagerie instantanée sécurisé et respectueux de la vie privée. Son architecture centrale est composée de **nœuds de service** formant un réseau pair à pair, maintenant une structure à deux niveaux : **réseau principal** et **sous-réseaux**. Les clients peuvent se connecter à n'importe quel nœud de service et réaliser une communication sécurisée entre appareils via un chiffrement de bout en bout. La conception du protocole suit le principe **zéro confiance** : les nœuds de service entre eux et entre nœuds de service et clients supposent que l'autre partie peut être malveillante, toutes les données critiques doivent être vérifiées cryptographiquement.

### 2. Identité des nœuds et découverte du réseau
- **Identifiant de nœud** : Chaque nœud de service possède une paire de clés Ed25519 à long terme, l'ID du nœud étant le hachage de la clé publique (`NodeID = blake3(public_key)`). La communication entre nœuds utilise le protocole Noise (mode XX) pour établir un canal crypté et vérifier mutuellement le NodeID.
- **Découverte du réseau** :
  - Les nœuds initiaux rejoignent le réseau via une liste préconfigurée de nœuds de départ.
  - Les nœuds maintiennent une table de hachage distribuée (DHT) de style Kademlia pour stocker les informations d'adresse des autres nœuds (IPv4/IPv6 + port) et la détection de vivacité.
  - Les nœuds publient régulièrement leur propre adresse dans la DHT et actualisent la liste des voisins.
  - Chaque nœud doit prendre en charge le protocole STUN pour faciliter la traversée NAT ; si la traversée est impossible, ils peuvent relayer via d'autres nœuds (fonctionnalité TURN-like, mais les nœuds de relais ne font que transférer des données cryptées).

### 4. Identité et authentification des utilisateurs
- **Identité globale** : Chaque utilisateur possède un UUID global `auth_id`, ainsi qu'une paire de clés Ed25519 à long terme (utilisée pour la signature d'identité). L'utilisateur génère cette identité lors de sa première utilisation, et la clé publique est signée et publiée par l'utilisateur lui-même. **auth_id est le secret le plus élevé du protocole Skynet et ne doit jamais être divulgué ou stocké dans les données internes du sous-réseau.**
- **Modèle de sous-réseau** :
  - Chaque système de communication ou communauté indépendant est un sous-réseau indépendant
  - Les utilisateurs ont des listes de contacts différentes dans différents sous-réseaux
  - `auth_id` est similaire à un vrai nom, identifiant unique l'utilisateur à travers les sous-réseaux, **utilisé uniquement pour l'authentification de l'identité de l'utilisateur, pas dans les données internes du sous-réseau**
  - `user_id` est similaire à un pseudo/nom d'écran, identifiant local dans le sous-réseau, utilisé pour la protection de la vie privée, **toutes les données internes du sous-réseau utilisent uniquement user_id**
  - Les utilisateurs n'utilisent que `user_id` comme identifiant dans le sous-réseau
  - La clé publique de l'utilisateur dans le sous-réseau (peut utiliser la clé publique d'identité globale ou générer séparément une clé dédiée au sous-réseau) doit être signée par lui-même avant d'être soumise au nœud de maintenance du sous-réseau
  - Les utilisateurs peuvent rejoindre plusieurs sous-réseaux en même temps, en utilisant le même `auth_id` pour l'authentification
  - **Type de sous-réseau** : Chaque sous-réseau a un identifiant de type (par exemple `organization`, `community`, `private`, `custom`), le client décide en fonction du type de charger et comment traiter
  - **Métadonnées du sous-réseau** : Chaque sous-réseau contient des métadonnées (type, version, nom, description, fonctionnalités, etc.), utilisées pour l'identification et l'adaptation du client
- **Authentification du client** :
  - Lorsque le client se connecte au nœud de service, il doit fournir `auth_id` et terminer un défi-réponse : le nœud de service envoie un nombre aléatoire, le client le signe avec sa clé privée d'identité et le renvoie, le nœud de service vérifie la signature.
  - Après authentification réussie, le nœud de service génère une clé de session temporaire pour ce client (utilisée pour le chiffrement des demandes ultérieures) et maintient l'état en ligne du client pendant la durée de validité de la session.

### 5. Chiffrement de bout en bout
Tout le contenu de communication entre utilisateurs (messages privés, messages de groupe, transferts de fichiers) utilise un chiffrement de bout en bout, les nœuds de service ne peuvent pas le déchiffrer. Le schéma de chiffrement s'inspire du protocole Signal et du protocole MLS :
- **Clé d'appareil** : Chaque appareil client génère une paire de clés X25519 comme identifiant d'appareil, la signe avec la clé privée d'identité de l'utilisateur et la publie dans le sous-réseau auquel il appartient (en tant que partie de la liste des membres). Les utilisateurs peuvent avoir plusieurs appareils.
- **Messages privés** :
  - Les deux parties établissent une clé partagée via le protocole d'échange de clés X3DH et utilisent l'algorithme Double Ratchet pour générer des clés de session, réalisant la sécurité avant et arrière.
  - Le message d'échange de clés initial peut être transféré via le nœud de service (crypté), le nœud de service ne fait que transmettre le texte chiffré.
- **Chats de groupe** :
  - Utilise le protocole MLS (Messaging Layer Security), fournissant une confidentialité avant, une confidentialité arrière et une rotation efficace des clés lors des changements de membres.
  - Le protocole MLS gère automatiquement les mises à jour de clés lors de l'ajout/suppression de membres, sans intervention manuelle de l'administrateur.
- **Protection des métadonnées** :
  - Le nœud de service ne peut voir que le destinataire du message (sous-réseau cible et utilisateur/groupe), mais ne peut pas voir le contenu du message. Pour cacher le destinataire, on peut envisager d'utiliser une technologie de réseau mixte (comme Loopix), mais la version initiale ne l'implémente pas, laissant cela à une extension ultérieure.

### 5.1 Paramètres de sécurité (priorité à l'efficacité)
Le protocole Skynet utilise les paramètres de sécurité suivants, priorisant les performances et l'expérience utilisateur tout en garantissant une sécurité de niveau entreprise :
| Paramètre | Valeur | Description |
|-----------|--------|-------------|
| **Algorithme de signature** | Ed25519 | Signature d'identité de nœud et d'utilisateur |
| **Échange de clés** | X25519 | Utilisé par X3DH et MLS |
| **Algorithme de hachage** | Blake3-256 | Sortie de 256 bits, utilisé pour NodeID, content_hash, etc. |
| **Chiffrement symétrique** | ChaCha20-Poly1305 | Chiffrement de message, clé de 256 bits |
| **Protocole Noise** | Noise_XX_25519_ChaChaPoly_BLAKE2s | Communication entre nœuds |
| **Nombre de pré-clés X3DH** | 100 | Nombre de pré-clés à usage unique prégénérées |
| **Suite cryptographique MLS** | MLS10_128_HPKEX25519_AES128GCM_SHA256_Ed25519 | Suite cryptographique du protocole MLS |

### 5.2 Principes de conception
Le protocole Skynet suit les principes de conception fondamentaux suivants :
1. **Priorité à l'efficacité** : Prioriser les performances du système et l'expérience utilisateur tout en garantissant la sécurité de base
2. **Solution moderne** : Utiliser des primitives et protocoles cryptographiques modernes et suffisamment vérifiés
3. **Conception extensible** : L'architecture prend en charge l'extension fluide des mécanismes de sécurité futurs
4. **Sécurité pratique** : Fournir une garantie de sécurité de niveau entreprise, sans poursuivre une conception excessive de niveau militaire

### 5.3 Protection contre les attaques par rejeu
Tous les messages cryptés contiennent un mécanisme anti-rejeu :
- **Numéro de séquence de message** : Chaque message contient un numéro de séquence entier non signé de 64 bits strictement croissant
- **Fenêtre de rejeu** : Le récepteur maintient une fenêtre glissante des 1000 derniers numéros de séquence de message
- **Logique de vérification** :
  1. Si le numéro de séquence ≤ la valeur minimale de la fenêtre, rejeter
  2. Si le numéro de séquence est dans la fenêtre et existe déjà, rejeter
  3. Sinon accepter le message et mettre à jour la fenêtre
- **Protection côté serveur** : Les nœuds de maintenance vérifient également le numéro de séquence pour empêcher les clients malveillants de rejouer des messages

### 6. Stockage et synchronisation des données
Les données du sous-réseau sont stockées collectivement par un ensemble de nœuds de maintenance, garantissant la fiabilité et la cohérence même en cas de malice de certains nœuds.
- **Modèle de données** :
  - Toutes les données du sous-réseau sont stockées sous la forme d'un **journal d'opérations immuable**, chaque opération (comme l'ajout d'un membre, la création d'une ressource, l'envoi d'un message) est signée par l'initiateur et contient un numéro de séquence strictement croissant.
  - Le journal converge finalement vers un état déterministe (peut être reconstruit en rejouant les opérations).
  - **Données de ressource** : Incluent les métadonnées et les hachages de contenu des ressources comme les fichiers, les images, etc., le contenu de la ressource peut être stocké dans un stockage distribué (comme IPFS) ou dans un stockage local.
  - **Données de message** : Incluent le contenu du message, l'expéditeur, le destinataire, l'horodatage, etc., prennent en charge le chiffrement de bout en bout.
- **Protocole de réplication** :
  - Les nœuds de maintenance échangent des journaux d'opérations via le protocole Gossip, chaque nœud vérifie indépendamment les signatures et les autorisations.
  - Utilise des **horloges vectorielles** ou des **vecteurs de version** pour détecter les conflits, les règles de résolution des conflits sont définies par la politique du sous-réseau (comme "priorité aux opérations de l'administrateur" ou "priorité à l'horodatage").
  - Les nœuds prennent régulièrement des instantanés de hachage du journal et les comparent avec d'autres nœuds, si une incohérence est découverte (comme un nœud falsifiant malveillamment l'historique), les autres nœuds peuvent le marquer comme non fiable et déclencher une intervention de l'administrateur.
- **Opérations d'écriture** :
  - Le client soumet une opération d'écriture (comme l'invitation d'un membre) à n'importe quel nœud de service, qui transmet la demande à tous les nœuds de maintenance.
  - Après avoir vérifié les autorisations et la signature, les nœuds de maintenance ajoutent l'opération au journal local et la diffusent à d'autres nœuds de maintenance.
  - Le client peut demander une confirmation de la majorité des nœuds de maintenance pour considérer l'opération réussie, ou utiliser une vérification "lecture après écriture" : lire l'état le plus récent depuis plusieurs nœuds de maintenance, alerter en cas d'incohérence.
- **Opérations de lecture** :
  - Le client peut lire les données du sous-réseau depuis n'importe quel nœud de maintenance, mais doit les obtenir depuis plusieurs nœuds et les comparer, en utilisant le résultat cohérent avec la majorité ; ou ne faire confiance qu'aux données signées par suffisamment de nœuds (comme une signature combinée).
  - Pour améliorer les performances, le client peut mettre en cache les données et interroger régulièrement les mises à jour.

### 7. Flux de transmission des messages
Prendre l'exemple de l'utilisateur A envoyant un message privé à l'utilisateur B :
1. Le client de A se connecte à un certain nœud de service S (peut être choisi arbitrairement).
2. Le client envoie une demande à S : obtenir la liste des clés publiques des appareils de B dans le sous-réseau cible. S l'obtient depuis les nœuds de maintenance de ce sous-réseau et la renvoie (doit être comparée par plusieurs nœuds, ou faire confiance au paquet de clés signé par B).
3. A utilise la clé publique de B pour exécuter la poignée de main X3DH, génère une clé de session et chiffre le contenu du message.
4. A envoie le paquet de message chiffré (contenant l'identifiant du destinataire : subnet_id, user_id de B) à S.
   - **Principe d'isolation du sous-réseau** : Les messages ne sont transmis que dans le même sous-réseau, pas de routage inter-sous-réseaux.
5. S trouve l'ensemble des nœuds de maintenance de ce sous-réseau en fonction de subnet_id et transmet le message à ces nœuds (ou choisit un nœud pour stocker le message hors ligne).
6. Si B est en ligne, un certain nœud de service auquel il est connecté recevra le message via Gossip ou un mécanisme d'abonnement et le poussera à B ; si B est hors ligne, le nœud de maintenance stocke le message et attend que B se connecte pour le récupérer.
7. Après avoir reçu le texte chiffré, B utilise sa propre clé privée d'appareil pour le déchiffrer et envoie un accusé de réception (optionnel).

Les messages de groupe sont similaires, mais utilisent une clé symétrique de groupe pour le chiffrement, le destinataire est channel_id, le nœud de maintenance distribue le message à tous les membres en ligne du groupe (ou le stocke comme historique de chat de groupe).

**Transfert inter-sous-réseaux** : Si vous devez transférer un message vers un autre sous-réseau, vous devez le copier dans le sous-réseau cible plutôt que de le référencer directement. Après la copie, le message dans le sous-réseau cible a un cycle de vie indépendant.

### 8. Considérations de sécurité et réponse aux hypothèses malveillantes
- **Nœud de service malveillant** :
  - **Falsification des données** : Toutes les données importantes (liste des membres, clés publiques) sont signées par l'initiateur, le client peut vérifier la signature et les obtenir depuis plusieurs nœuds pour une comparaison croisée.
  - **Déni de service** : Le client peut basculer vers d'autres nœuds de service connectés ; si les nœuds de maintenance du sous-réseau agissent collectivement malveillamment, le client peut le découvrir via des canaux sociaux et reconstruire le sous-réseau.
  - **Écoute des métadonnées** : Le chiffrement de bout en bout protège le contenu, mais le nœud de service peut toujours voir la relation de communication (qui discute avec qui). Un réseau Mix peut être introduit ultérieurement pour cacher la relation.
- **Client malveillant** :
  - **Abus des ressources** : Le nœud de service impose une limitation de débit à chaque client, peut demander au client d'effectuer une preuve de travail basée sur la difficulté (comme Hashcash) ou de payer un micro-token (si un modèle économique est introduit).
  - **Usurpation d'identité** : Toutes les opérations du client doivent être signées avec la clé privée d'identité, le nœud de service vérifie la signature pour empêcher la falsification.
  - **Attaque Sybil** : Rejoindre un sous-réseau nécessite une vérification par l'administrateur, ce qui peut empêcher les attaquants de créer de nombreuses fausses identités ; l'enregistrement d'identité globale peut envisager de combiner des points d'ancrage externes (comme la vérification par e-mail) mais ce n'est pas obligatoire.
- **Collusion des nœuds de maintenance du sous-réseau** :
  - Si plus de la moitié des nœuds de maintenance du sous-réseau sont malveillants, ils peuvent falsifier les données ou refuser le service. Pour cela, le créateur du sous-réseau peut demander aux nœuds de maintenance de fournir une garantie lors de leur sélection (comme le verrouillage de Token), ou au niveau du protocole permettre au client d'obtenir des données depuis un tiers de confiance (comme la vérification du graphe social). La conception peut introduire un mécanisme de "nœud de témoin", permettant aux nœuds ordinaires de s'abonner aux données du sous-réseau et de générer des preuves de cohérence, le client peut dépendre de ces nœuds de témoin.

### 9. Aperçu de l'interaction client-serveur (référence)
La fonctionnalité client fait référence aux produits IM, les interactions principales incluent :
- **Connexion** : Fournir `auth_id` et un défi de signature, obtenir un jeton de session.
- **Changement de sous-réseau** : Le client répertorie les sous-réseaux que l'utilisateur a rejoints (cache local), après changement, l'interface utilisateur affiche le contexte correspondant.
- **Contacts** : Obtenir la liste des utilisateurs depuis les données du sous-réseau, prendre en charge la recherche.
- **Envoi de message** : Après chiffrement de bout en bout, envoyer au nœud de service, qui route vers le nœud de maintenance du sous-réseau cible.
- **Réception de message** : Recevoir des notifications depuis le nœud de service via un long polling ou WebSocket.
- **Gestion de groupe** : Créer un groupe, inviter des membres, modifier les informations du groupe, les opérations doivent être signées et soumises au nœud de maintenance du sous-réseau.
- **Gestion des ressources** : Télécharger, téléverser, mettre à jour, supprimer des ressources, le hachage de contenu de la ressource est stocké dans les données du sous-réseau, le contenu peut être stocké dans un stockage distribué (comme IPFS ou similaire).
- **Transfert de fichier** : Le fichier est téléchargé en tant que ressource, puis la référence de la ressource est envoyée en tant que message.

### 10. Limites du protocole et clause de non-responsabilité

Le protocole Skynet définit clairement le contenu **non impliqué et non responsable** suivant :

| Catégorie | Description | Responsable |
|-----------|-------------|-------------|
| **Gestion des clés** | Stockage, sauvegarde, récupération de la clé privée d'identité et de la clé d'appareil | Client |
| **Génération de nombres aléatoires** | Implémentation du CSPRNG (générateur de nombres pseudo-aléatoires cryptographiquement sécurisé) et source d'entropie | Client |
| **Dérivation de clés** | Algorithme de dérivation de clés à partir d'un mot de passe (comme PBKDF2, Argon2) | Client |
| **Module de sécurité matériel** | Utilisation de HSM, élément sécurisé (SE), environnement d'exécution de confiance (TEE) | Client |
| **Authentification biométrique** | Méthodes biométriques comme l'empreinte digitale, la reconnaissance faciale | Client |
| **Cryptographie post-quantique** | Migration vers la cryptographie post-quantique (PQC) et mécanisme d'encapsulation de clés | Hors sujet, pas impliqué pour le moment |
| **Réseau mixte** | Technologies de masquage des métadonnées comme Loopix | Extension optionnelle, pas implémentée dans la version initiale |
| **Incitation économique** | Modèles économiques comme Token, garantie, preuve de travail | Extension optionnelle |
| **Mécanisme de gouvernance** | Gouvernance décentralisée comme DAO, vote, arbitrage | Extension optionnelle |

**Remarque importante** :
- Le protocole Skynet ne définit que les spécifications de communication réseau et de structure de données
- Les implémenteurs de clients doivent garantir eux-mêmes la bonne implémentation des mécanismes de sécurité ci-dessus
- La sécurité cryptographique dépend finalement de la bonne implémentation du client et de la conscience de sécurité de l'utilisateur

### 10. Résumé
Le protocole Skynet fournit une base pour construire un système de communication respectueux de la vie privée similaire à l'IM d'entreprise via un réseau de nœuds de service décentralisés, l'isolation des sous-réseaux, le chiffrement de bout en bout et la réplication de données tolérante aux fautes byzantines. La conception du protocole prend pleinement en compte le scénario de méfiance mutuelle entre le serveur et le client, garantissant que la propriété et le contrôle des données utilisateur sont entre leurs propres mains.

#### Positionnement de sécurité
L'objectif de sécurité du protocole Skynet est **la sécurité pratique de niveau entreprise**, pas une conception excessive de niveau militaire :
- ✅ Protéger la vie privée des communications d'entreprise et la sécurité des données
- ✅ Résister aux attaques réseau courantes et aux menaces internes
- ✅ Fournir de bonnes performances et une bonne expérience utilisateur
- ❌ Ne pas poursuivre la sécurité extrême contre des adversaires de niveau État
- ❌ Ne pas sacrifier la disponibilité en échange d'une sécurité théorique absolue

#### Directions d'extension futures
La conception du protocole réserve de l'espace d'extension pour les futures mises à niveau de sécurité :
1. **Protection des métadonnées** : Peut introduire une technologie de réseau mixte pour cacher les relations de communication
2. **Migration post-quantique** : Prend en charge une transition fluide vers des algorithmes de cryptographie post-quantique
3. **Amélioration du consensus** : Peut mettre à niveau l'algorithme de consensus et le seuil de sécurité selon les besoins
4. **Amélioration de l'audit** : Peut étendre une audit de sécurité et des fonctionnalités de conformité plus fines
