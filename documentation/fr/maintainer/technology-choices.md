# Choix technologiques

## Aperçu

Ce document enregistre les décisions de choix technologiques du système AI Company, y compris le protocole de communication, le mécanisme de découverte, les normes de sécurité, etc.

## Communication entre nœuds

### Mécanisme de découverte

Le système adopte un mécanisme de découverte de nœuds multicouche :

| Méthode de découverte | Choix technologique | Utilisation |
|---------|---------|------|
| **Découverte de réseau local** | mDNS/Bonjour | Découverte automatique des nœuds du réseau local |
| **Découverte cloud** | Service de répertoire cloud | Découverte des nœuds distants |
| **Connexion manuelle** | Saisie directe d'adresse | Spécification manuelle de la connexion du nœud |

### Protocole de communication

| Type de protocole | Choix technologique | Utilisation |
|---------|---------|------|
| **Communication en temps réel** | WebSocket | Communication bidirectionnelle en temps réel |
| **Appel de service** | gRPC | Appel de service efficace entre services |
| **Synchronisation de fichiers** | rsync ou protocole similaire | Synchronisation incrémentielle de fichiers |

### Communication sécurisée

| Mécanisme de sécurité | Choix technologique | Description |
|---------|---------|------|
| **Chiffrement de transmission** | TLS 1.3 | Chiffrement de toutes les communications |
| **Authentification de nœud** | Certificat de nœud | Authentification bidirectionnelle |
| **Chiffrement de données** | Chiffrement de bout en bout | Chiffrement de bout en bout des données sensibles |

## Technologies liées aux nœuds de travail

### Prise en charge des types de nœuds

Le système prend en charge les types de nœuds de travail suivants :
- PC personnel Windows
- Serveur
- Téléphone Android
- Téléphone iOS
- Tablette
- Appareil IoT

### État des nœuds

| État | Description |
|------|------|
| **Online** | Le nœud est en ligne et inoccupé |
| **Busy** | Le nœud est en ligne mais occupé |
| **Offline** | Le nœud est hors ligne |
| **Maintenance** | Le nœud est en maintenance |

## Choix technologiques de stockage

Voir [data-models.md](./data-models.md) pour comprendre l'architecture de stockage détaillée et les choix technologiques.

### Choix de base de données

| Base de données | Utilisation | Scénario |
|-------|------|------|
| **SQLite** | Base de données relationnelle légère | Déploiement sur une seule machine |
| **PostgreSQL** | Base de données relationnelle de niveau entreprise | Environnement de production |

### Choix de stockage d'objets

| Type de stockage | Utilisation | Scénario |
|---------|------|------|
| **Système de fichiers (FS)** | Stockage de fichiers locaux | Déploiement sur une seule machine |
| **S3** | Service de stockage d'objets | Environnement de production et déploiement cloud |

## Principes de choix technologiques

1. **Priorité à l'open source** : Prioriser les technologies open source matures
2. **Normalisation** : Suivre les normes de l'industrie et les meilleures pratiques
3. **Extensibilité** : Choisir des technologies prenant en charge l'extension horizontale
4. **Sécurité** : Prioriser la sécurité et la protection de la vie privée
5. **Performance** : S'assurer que les choix technologiques répondent aux exigences de performance
