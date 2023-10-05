GRANT ALL PRIVILEGES ON *.* TO 'automato'@'%';

FLUSH PRIVILEGES;
CREATE DATABASE study;
USE study;
CREATE TABLE `author` (
  `id` bigint NOT NULL AUTO_INCREMENT COMMENT 'Primary Key',
  `name` varchar(255) NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=83 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;
INSERT INTO `author` VALUES (1,'Augusto Cury'),(3,'Chuck Wendig'),(72,'Paulo Coelho'),(73,'Ana Cristina Vargas'),(74,'Arthur Conan Doyle'),(75,'Augusto'),(79,'teste'),(80,'tes');
DROP TABLE IF EXISTS `books`;
CREATE TABLE `books` (
  `id` bigint NOT NULL AUTO_INCREMENT COMMENT 'Primary Key',
  `id_publishing_company` bigint NOT NULL,
  `id_author` bigint NOT NULL,
  `title` varchar(255) NOT NULL,
  `category` varchar(255) NOT NULL,
  `edition` varchar(100) NOT NULL,
  `publish_date` varchar(255) NOT NULL,
  `description` varchar(255) NOT NULL,
  `qnt_specimens` int NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=51 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;
INSERT INTO `books` VALUES (1,1,1,'As Armadilhas da Mente','Romance','1','2013','Camile é uma mulher bvela, rica e brilhante, capaz de deixar as pessoas impressionadas com sua habilidade de debate e argumentar. \nMas seus diplomas e seu intelecto não foram suficientes para evitar que se tornasse vítima de suas próprias emoçoes.',1),(2,3,3,'Star Wars: Marcas da Guerra','Ficção Científica','1','2015','Às vésperas de STAR WARS: O despertar da Força, os fãs têm a oportunidade de\n                        acompanhar histórias inéditas ambientadas no universo criado por GeorgeLucas.',1),(40,63,1,'O Vendedor de Sonhos: O Chamado','Ficção Brasileira','5','2021',' O romance mais vendido de Augusto Cury, que deu origem ao filme de Jayme Monjardim, com Dan Stulbach e César Troncoso nos papéis principais. Edição revista pelo autor, com passagens inéditas presentes na adaptação para o cinema.',1),(41,64,72,'Ser como o Rio que Flui...','Ficção Brasileira','1','2007',' Coleção Paulo Coelho',1),(42,64,72,'A Bruxa de Portobello','Ficção Brasileira','1','2007',' Coleção - Antes que todos estes depoimentos saíssem da minha mesa de trabalho e seguissem o destino que eu havia determinado para eles, pensei em transformá-los em um livro tradicional, onde uma estória real é contada depois de exaustiva pesquisa.',1),(43,64,72,'Brida','Ficção Brasileira','1','2007',' Coleção Paulo Coelho',1),(44,65,73,'O Bispo','Religião e Espiritualidade','1','2014',' Até que ponto você aceitaria explorar sua humanidade? Quanto do que pensa, sente ou vive aceitaria tornar público? Ricardo aceitou o desafio e revela-se aos leitores contando segredos que levou para o túmulo.',1),(45,66,74,'Sherlock Holmes: O Vale do Medo','Drama','1','2018',' \"Este é o Vale do Medo, o vale da morte. O terror está nos corações das pessoas do crepúsculo ao redor do dia.\" É nesse tom sombrio que se desenrola a surpreendente narrativa deste último caso investigado por Sherlock Holmes.',1);
DROP TABLE IF EXISTS `permissions`;
CREATE TABLE `permissions` (
  `id` int NOT NULL AUTO_INCREMENT COMMENT 'Primary Key',
  `level` varchar(50) DEFAULT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=4 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;
INSERT INTO `permissions` VALUES (1,'admin'),(2,'librarian'),(3,'user');
DROP TABLE IF EXISTS `publishing_company`;
CREATE TABLE `publishing_company` (
  `id` bigint NOT NULL AUTO_INCREMENT COMMENT 'Primary Key',
  `name` varchar(255) NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=75 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;
INSERT INTO `publishing_company` VALUES (1,'Arqueiro'),(3,'Aleph'),(63,'Planeta'),(64,'gold'),(65,'Vida & Concsciência'),(66,'Lafonte'),(67,'Arque'),(71,'teste'),(72,'tes');
DROP TABLE IF EXISTS `users`;
CREATE TABLE `users` (
  `id` bigint NOT NULL AUTO_INCREMENT COMMENT 'Primary Key',
  `id_permission` int NOT NULL,
  `id_ebook` bigint DEFAULT '0',
  `name` varchar(50) DEFAULT NULL,
  `surname` varchar(255) DEFAULT NULL,
  `password` varchar(255) DEFAULT NULL,
  `email` varchar(255) DEFAULT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=113 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;
INSERT INTO `users` VALUES (7,1,0,'adm','nistrador','admin','admin@admin.com'),(107,2,0,'bib','liotecario','123','bib@liotecario.com'),(110,3,0,'user','comum','user123','user@user.com');