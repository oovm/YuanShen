# Conception de la Couche de Protocole

Ce document décrit en détail la couche de protocole du système AI Company, c'est-à-dire le module de protocole Skynet dans le répertoire protocols/. La couche de protocole est l'infrastructure de l'ensemble du système, fournissant des protocoles de communication standardisés, des définitions de types de données et des interfaces d'interaction.

## Aperçu de la Couche de Protocole

La couche de protocole est la couche la plus basse de l'architecture à trois couches d'AI Company, fournissant des services de base et des protocoles d'interaction standardisés pour la couche d'implémentation supérieure (modules centraux augur-*) et la couche de présentation.

### Rôle de la Couche de Protocole dans l'Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                  Couche de Présentation (Côté Application)    │
├─────────────────────────────────────────────────────────────┤
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐    │
│  │  ai-company  │  │  ai-empire   │  │  ai-planet   │    │
│  └──────────────┘  └──────────────┘  └──────────────┘    │
└─────────────────────────────────────────────────────────────┘
                            ↓ Appel
┌─────────────────────────────────────────────────────────────┐
│          Couche d'Implémentation (Modules Centraux augur-*)   │
├─────────────────────────────────────────────────────────────┤
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐    │
│  │augur-agent   │  │augur-orchestr│  │augur-organiz │    │
│  └──────────────┘  └──────────────┘  └──────────────┘    │
└─────────────────────────────────────────────────────────────┘
                            ↓ Utilise
┌─────────────────────────────────────────────────────────────┐
│          ← Couche de Protocole (Protocole Skynet) →          │
├─────────────────────────────────────────────────────────────┤
│  Types de base  Protocole d'authentification  Protocole de chat  Protocole de passerelle  Protocole de mémoire  Protocole de service  │
│  Protocole de pont  Protocole de notification  Protocole de persistance  Protocole de stockage                      │
└─────────────────────────────────────────────────────────────┘
```

### Responsabilités Principales

Les responsabilités principales de la couche de protocole incluent :

1. **Fournir des définitions de types de données standardisées** : Unifier le format d'échange de données entre les modules
2. **Définir les protocoles de communication** : Normaliser la méthode d'interaction entre les modules et les services
3. **Fournir des mécanismes de sécurité de base** : Authentification d'identité, transmission cryptée, contrôle d'accès
4. **Prendre en charge la communication décentralisée** : Structure à deux niveaux réseau principal-sous-réseau, communication réseau pair à pair
5. **Abstraire les différences de niveau inférieur** : Fournir une interface unifiée pour la couche supérieure, masquant les détails d'implémentation de niveau inférieur

## Détails des Modules de Protocole

### 1. skynet-types - Définitions de Types de Base

**Responsabilités et Fonctions** :
- Fournir toutes les définitions de types partagés de la couche de protocole Skynet
- Définir les structures de données de base, les énumérations et les constantes
- Fournir un mécanisme de génération et de traitement d'identifiants
- Définir les types d'erreur et les spécifications de gestion d'erreur unifiées
- Fournir les types de messages JSON-RPC et WebSocket

**Sous-modules Principaux** :
- `agent` - Types liés aux agents
- `bridge` - Types liés au pont
- `chat` - Types liés au chat
- `error` - Types liés aux erreurs
- `id` - Types liés aux identifiants
- `jsonrpc` - Types liés à JSON-RPC
- `memory` - Types liés à la mémoire
- `org` - Types liés à l'organisation
- `resource` - Types liés aux ressources
- `subnet` - Types liés aux sous-réseaux
- `user` - Types liés aux utilisateurs
- `utils` - Fonctions utilitaires
- `websocket` - Types liés à WebSocket

**Rôle dans l'Architecture** : Servir de base à tous les autres modules de protocole, fournir un système de types unifié, garantir la compatibilité des données entre les modules.

---

### 2. skynet-auth - Protocole d'Authentification

**Responsabilités et Fonctions** :
- Fournir une interface d'authentification d'identité utilisateur
- Gérer les autorisations et le contrôle d'accès
- Gérer la gestion de session et les jetons
- Prendre en charge plusieurs méthodes d'authentification

**Fonctionnalités Principales** :
- Définition du flux d'authentification utilisateur
- Gestion des autorisations et mécanisme d'autorisation
- Génération et vérification des jetons de session
- Gestion du cycle de vie des jetons

**Rôle dans l'Architecture** : Fournir une base d'authentification d'identité et de contrôle d'accès sécurisée pour l'ensemble du système, garantissant que seuls les utilisateurs et services autorisés peuvent accéder aux ressources du système.

---

### 3. skynet-chat - Protocole de Chat

**Responsabilités et Fonctions** :
- Fournir un chat en temps réel et une gestion de session
- Définir les types et formats de messages
- Prendre en charge l'historique de chat
- Fournir une interface de communication en temps réel

**Fonctionnalités Principales** :
- Gestion des sessions (canaux)
- Définition de plusieurs types de messages (texte, fichier, image, etc.)
- Notification de message en temps réel
- Recherche et récupération de l'historique de chat

**Rôle dans l'Architecture** : Fournir un support de communication de base pour les fonctionnalités de collaboration d'AI Company, y compris les canaux de projet, la communication d'équipe, etc.

---

### 4. skynet-gateway - Protocole de Passerelle

**Responsabilités et Fonctions** :
- Fournir une interface de passerelle API
- Traiter le routage et le transfert des demandes
- Implémenter l'équilibrage de charge
- Exécuter la vérification de sécurité

**Fonctionnalités Principales** :
- Routage et distribution des demandes
- Découverte de service et équilibrage de charge
- Vérification des demandes et filtrage de sécurité
- Gestion de version API

**Rôle dans l'Architecture** : Servir de point d'entrée du système, gérer uniformément les demandes externes, fournir un canal d'accès API sécurisé et efficace.

---

### 5. skynet-memory - Protocole de Mémoire

**Responsabilités et Fonctions** :
- Fournir une interface de stockage et de récupération de mémoire
- Définir les types de données de mémoire
- Prendre en charge la recherche et la requête de mémoire
- Gérer les étiquettes et la classification de la mémoire

**Fonctionnalités Principales** :
- Stockage et lecture des données de mémoire
- Recherche vectorielle de mémoire
- Gestion des étiquettes et classification
- Association et contexte de la mémoire

**Rôle dans l'Architecture** : Fournir un support de système de mémoire pour les agents, permettant aux agents de se souvenir des interactions historiques, d'apprendre de l'expérience et de former une mémoire à long terme.

---

### 6. skynet-service - Protocole de Service

**Responsabilités et Fonctions** :
- Fournir des définitions d'interface de service générales
- Prendre en charge le mécanisme de découverte de service
- Implémenter la vérification de santé et la surveillance
- Gérer l'enregistrement et la désinscription des services

**Fonctionnalités Principales** :
- Spécification d'interface de service unifiée
- Enregistrement et découverte de service
- Surveillance de l'état de santé
- Gestion du cycle de vie du service

**Rôle dans l'Architecture** : Fournir un mécanisme d'enregistrement, de découverte et de communication unifié pour divers services du système, prendre en charge l'architecture de microservices.

---

### 7. skynet-bridge - Protocole de Pont

**Responsabilités et Fonctions** :
- Fournir une interface d'intégration de système externe
- Traiter le transfert de messages et la conversion de protocole
- Prendre en charge l'intégration multi-plateforme
- Gérer la communication inter-systèmes

**Fonctionnalités Principales** :
- Pont de système externe
- Conversion et adaptation de protocole
- Routage et transfert de messages
- Connecteur multi-plateforme

**Rôle dans l'Architecture** : Permettre à AI Company de s'intégrer et de communiquer avec des systèmes externes (comme des services tiers, des systèmes hérités, etc.).

---

### 8. skynet-notification - Protocole de Notification

**Responsabilités et Fonctions** :
- Fournir un service de notification et de push de message
- Définir les types et formats de notification
- Prendre en charge plusieurs canaux de push
- Gérer l'historique des notifications

**Fonctionnalités Principales** :
- Interface de service de notification
- Push multi-canaux (mobile, bureau, e-mail, etc.)
- Définition des types de notification
- Historique et gestion d'état des notifications

**Rôle dans l'Architecture** : Fournir une capacité de notification en temps réel pour le système, garantissant que les utilisateurs obtiennent rapidement des informations et mises à jour importantes.

---

### 9. skynet-persistence - Protocole de Persistance

**Responsabilités et Fonctions** :
- Fournir une interface d'entrepôt de données et de persistance
- Définir les types d'entités et le modèle d'entrepôt
- Prendre en charge la requête de données et la gestion de transaction
- Fournir une couche d'accès aux données unifiée

**Fonctionnalités Principales** :
- Définition d'interface d'entrepôt
- Spécification des types d'entités
- Requête et filtrage de données
- Gestion de transaction et garantie de cohérence

**Rôle dans l'Architecture** : Fournir une abstraction de persistance de données unifiée pour le système, masquer les différences de bases de données sous-jacentes, prendre en charge plusieurs backends de stockage.

---

### 10. skynet-storage - Protocole de Stockage

**Responsabilités et Fonctions** :
- Fournir une interface de service de stockage général
- Prendre en charge plusieurs types de stockage (fichier, Blob, objet, etc.)
- Gérer les autorisations de stockage et le contrôle d'accès
- Définir les spécifications d'opération de stockage

**Fonctionnalités Principales** :
- Interface de service de stockage général
- Prise en charge de plusieurs types de stockage
- Contrôle d'accès et gestion des autorisations
- Définition des opérations de stockage (téléchargement, téléversement, suppression, etc.)

**Rôle dans l'Architecture** : Fournir une abstraction de stockage de fichiers et d'objets unifiée pour le système, prendre en charge diverses exigences de stockage.

## Principes de Conception de la Couche de Protocole

### 1. Légèreté et Flexibilité

La couche de protocole ne définit que les interfaces et types **minimum nécessaires**, en restant légère et flexible. Les détails d'implémentation spécifiques sont implémentés de manière indépendante par les modules supérieurs en fonction des exigences.

### 2. Standardisation et Cohérence

Tous les modules de protocole suivent des spécifications de conception et des conventions de nommage unifiées, garantissant la cohérence et l'interopérabilité entre les modules.

### 3. Extensibilité

La conception du protocole réserve des points d'extension, prenant en charge l'ajout et la mise à niveau de fonctionnalités futures sans détruire les interfaces existantes.

### 4. Priorité à la Sécurité

La couche de protocole intègre des mécanismes de sécurité de base, y compris l'authentification d'identité, la transmission cryptée, le contrôle d'accès, etc., garantissant la sécurité du système.

### 5. Prise en Charge de la Décentralisation

La couche de protocole prend en charge la structure à deux niveaux réseau principal-sous-réseau, la communication réseau pair à pair, fournissant une infrastructure pour les applications décentralisées.

## Relations d'Interaction avec la Couche Supérieure

### Couche d'Implémentation (Modules augur-*)

Les modules centraux de la couche d'implémentation (comme augur-agent, augur-orchestrator, etc.) utilisent directement les interfaces et types fournis par la couche de protocole :

- `augur-agent` utilise `skynet-types` et `skynet-memory`
- `augur-organization` utilise `skynet-types` et `skynet-persistence`
- `augur-orchestrator` utilise `skynet-service` et `skynet-gateway`

### Couche de Présentation (Côté Application)

La couche de présentation utilise indirectement la couche de protocole via la couche d'implémentation, ou dans certains cas utilise directement les types et interfaces de base de la couche de protocole.

## Résumé

La couche de protocole est l'infrastructure du système AI Company, fournissant une base solide pour la couche d'implémentation supérieure et la couche de présentation via des protocoles de communication standardisés, des définitions de types de données et des interfaces d'interaction. Chaque module de protocole a ses propres responsabilités, formant conjointement un système de protocole complet, flexible et sécurisé, prenant en charge la conception décentralisée du système et les exigences d'application de niveau entreprise.

Grâce à l'abstraction de la couche de protocole, AI Company réalise :
- Conception modulaire, chaque partie peut évoluer indépendamment
- Interfaces standardisées, facilitant l'intégration et l'extension
- Base de sécurité, garantissant la sécurité du système
- Prise en charge de la décentralisation, réalisant la souveraineté des données et le contrôle utilisateur
