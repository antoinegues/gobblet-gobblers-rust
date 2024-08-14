# Gobblet Gobblers - Recréation en Rust

**Gobblet Gobblers** est un projet qui vise à reproduire le jeu de société édité par *Blue Orange*, tout en explorant les subtilités du langage de programmation **Rust**.

## 🎲 Aperçu du Jeu

**Gobblet Gobblers** est une version revisitée du classique jeu de morpion (ou tic-tac-toe). Chaque joueur dispose de six pièces de tailles différentes : 2 petites, 2 moyennes et 2 grandes. Les règles sont simples mais stratégiques :

- **Pose de Pièces** : Les joueurs placent leurs pièces sur une grille, avec la possibilité d'encapsuler (ou "gober") des pièces plus petites appartenant à l'adversaire.
- **Déplacement de Pièces** : Les joueurs peuvent déplacer leurs pièces déjà posées à condition que la destination contienne une pièce plus petite et que la pièce déplacée leur appartienne.
- **Objectif** : Alignez trois de vos pièces pour remporter la partie, que ce soit horizontalement, verticalement ou en diagonale.

## 🎯 Objectifs du Projet

Ce projet a pour but principal de :

- **Maîtriser Rust** : S'immerger dans les différentes facettes du langage Rust, en explorant tout, des concepts de base aux techniques avancées.
- **Créer un Terrain de Jeu pour l'Exploration** : Ce projet est conçu pour être un laboratoire d'expérimentation. Il est volontairement surdimensionné et complexe pour offrir un cadre riche où de nombreuses idées et technologies peuvent être testées et mises en pratique. Il ne sera donc jamais terminé, car son objectif est de rester un terrain de jeu en constante évolution.

## 🖥️ Interfaces Utilisateurs (IHMs)

Le projet inclut plusieurs Interfaces Homme-Machine (IHMs) pour permettre aux utilisateurs d'interagir avec le jeu de différentes manières. Celles-ci peuvent fonctionner simultanément selon les besoins et préférences des utilisateurs.

### Debug Console

L'IHM de Debug Console est l'outil parfait pour le développement et le débogage. Elle permet d'envoyer des commandes directement depuis la console et de visualiser en temps réel les événements générés par le modèle du jeu.

#### 🛠️ Commandes Disponibles

- **Nouvelle Partie** : Démarrez une nouvelle partie avec la commande suivante :
  ```
  1
  ```

- **Poser une Pièce** : Placez une pièce sur la grille à la position souhaitée en spécifiant les coordonnées `x` et `y`, ainsi que la taille de la pièce (`1` pour petite, `2` pour moyenne, `3` pour grande) :
  ```
  2 [x] [y] [size]
  ```

- **Déplacer une Pièce** : Déplacez une pièce d'une position à une autre en indiquant les coordonnées d'origine et de destination :
  ```
  3 [origin_x] [origin_y] [destination_x] [destination_y]
  ```
  
- **Quitter le jeu** : Quittez le jeu avec la commande suivante :
  ```
  4
  ```

> **Note** : Actuellement, seule l'IHM de Debug Console est implémentée, mais une IHM visuelle est en cours de développement.

---

## 📚 Ressources

- [Documentation du Jeu Gobblet Gobblers](https://www.blueorangegames.com/games/gobbletgobblers)