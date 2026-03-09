# Scénario 4 : Centre de service client automatisé

## Description du scénario

Une petite entreprise, utilisant le système AI Company pour construire un centre de service client automatisé, traitant les consultations client et les problèmes après-vente.

## Architecture organisationnelle

- Modèle **Petite équipe**
- **Secrétariat** : Responsable de la coordination globale et du contrôle de la qualité du service
- **Équipe de service client de première ligne** : Responsable de l'accueil direct des clients
  - Agent d'accueil : Accueillir les clients, comprendre les problèmes
  - Agent de classification : Juger le type de problème, allouer à l'équipe correspondante
- **Équipe de support technique** : Responsable du traitement des problèmes techniques
  - Agent technique : Répondre aux problèmes techniques, fournir des solutions
  - Agent d'escalade : Traiter les problèmes complexes, transférer à un humain si nécessaire
- **Équipe après-vente** : Responsable du traitement des problèmes après-vente
  - Agent après-vente : Traiter les retours, remboursements et autres problèmes
  - Agent de suivi : Suivre la satisfaction client
- **Équipe de base de connaissances** : Responsable de la maintenance de la base de connaissances
  - Agent de mise à jour : Mettre à jour le contenu de la base de connaissances
  - Agent d'analyse : Analyser les problèmes courants, optimiser la base de connaissances

## Flux de travail

1. **Accueil client** : L'agent d'accueil accueille le client, comprend le problème
2. **Classification du problème** : L'agent de classification juge le type de problème, l'alloue à l'équipe correspondante
3. **Traitement du problème** : L'agent de l'équipe correspondante traite le problème
4. **Escalade complexe** : Si le problème est complexe, l'agent d'escalade intervient ou transfère à un humain
5. **Suivi client** : L'agent de suivi suit la satisfaction client
6. **Mise à jour des connaissances** : L'agent d'analyse analyse les données, l'agent de mise à jour met à jour la base de connaissances

## Configuration clé

- Configurer un style de communication amical pour l'agent d'accueil
- Configurer les connaissances du produit et les réponses aux questions courantes pour l'agent technique
- Configurer le flux de mise à jour des connaissances et les normes de qualité pour l'équipe de base de connaissances
