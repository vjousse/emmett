<!-- 
.. title: Machine Learning & Big Data : des dangers pour les logiciels libres ?
.. slug: les-dangers-du-big-data-pour-les-logiciels-libres
.. date: 2017-04-18 06:16:04+02:00
.. tags: 
.. category: 
.. link: 
.. description: 
.. type: text
-->

<!-- https://color.adobe.com/Flat-UI-color-theme-2469224/edit/?copy=true&base=1&rule=Custom&selected=4&name=Copy%20of%20Flat%20UI&mode=rgb&rgbvalues=0.172549,0.243137,0.313725,0.905882,0.298039,0.235294,0.92549,0.941176,0.945098,0.203922,0.596078,0.858824,0.160784,0.501961,0.72549&swatchOrder=0,1,2,3,4 -->

Ou vous sortez de votre grotte ou vous avez forcément entendu parler de ces deux mots fourre-tout « __big data__ ». En gros, les informaticiens (mais aussi et surtout les personnes du marketing) ont tendance à l'utiliser dès qu'il faut __traiter un peu plus de deux lignes de données__ avec un programme informatique : autant dire tout le temps. C'est à la mode, ça fait bien en société et ça permet d'obtenir des financements French Tech.

À côté de ça, de réelles technologies dont vous ne pouvez plus vous passer sont basées sur ces concepts de « big data » et de « machine learning » (apprentissage artificiel) : la __reconnaissance de la parole__, les filtres __anti-spam__, la __recherche d'images__, la __traduction automatique__, j'en passe et des meilleurs. Et tout ça pourrait bien avoir de très __lourdes répercutions sur notre avenir__.

<!-- TEASER_END -->

## Les logiciels libres

Vous ne le savez peut-être pas, mais les [logiciels libres](https://fr.wikipedia.org/wiki/Logiciel_libre) sont __partout__. __Tous les sites que vous consultez au quotidien__ sont basés sur des logiciels libres (Twitter, Facebook, Google, …), les logiciels de __vos téléphones__ sont construits à partir de briques libres, vous pouvez __aller sur Internet__ grâce aux logiciels libres.

Rien de ce que l'on connait aujourd'hui n'aurait été possible sans le logiciel libre. Si [Richard Stallman](https://fr.wikipedia.org/wiki/Projet_GNU) n'avait pas initié le mouvement au début des années 80, le monde serait alors très différent.

Mais qu'est-ce qu'un logiciel libre au juste ? C'est beaucoup de choses à la fois (notamment un logiciel que l'on peut librement modifier et dupliquer), mais de mon point de vue, c'est surtout une vision du monde : __croire que l'avenir se construit en partageant plutôt qu'en gardant pour soi__.

J'aime cette vision du monde. J'aime me dire qu'__un jour chaque personne, chaque entreprise, aura plutôt intérêt à partager qu'à garder pour soi__. Notre modèle capitaliste actuel va totalement à l'encontre de ça, mais le logiciel libre est un exemple concret que ce n'est pas irréalisable.

## Le big data et l'apprentissage artificel

Vous allez me dire, que vient faire le big data ici ? Jusqu'ici, la __valeur ajoutée__ des logiciels se situait dans __le code source__ qui était produit par le(s) développeur(s) du logiciel lui-même. Ce code source, qui peut être mis sous licence libre, vous permet de vous servir du logiciel. Grâce au code de __Firefox ou de Chrome__ (et donc grâce au logiciel du même nom qui en découle), vous pouvez __aller sur Internet__. Grâce au code de __Linux__, vous pouvez utiliser vos téléphones __Androïd__.

Ce type de logiciel dont toute la valeur ajoutée (ou presque) dépend uniquement des lignes de code tapées par le développeur se prête très bien au monde du libre. Il suffit d'__un développeur talentueux__ pour initier un projet et il est ensuite assez aisé de __contribuer à plusieurs__ à distance sur ce même logiciel.

<div style="text-align:center;">
    <img alt="Schéma développement logiciel Open-Source" src="/images/schema_dev_opensource.png" />
</div>

C'est aussi simple que ça.

Mais ces dernières années, une __nouvelle vague de logiciels__ a vu le jour. Une partie de la valeur ajoutée du logiciel se situe toujours dans le code source, mais la plus grosse partie se situe maintenant dans les données que traite ce logiciel pour vous fournir ses fonctionnalités. Et c'est là que le bât blesse.

## Le monde des données

Une grosse partie des logiciels fonctionnant sur la base de machine learning ont __besoin de beaucoup données__. Si l'on veut vulgariser un peu, il faut qu'un __humain annote des données manuellement__ pour dire à la machine ce qu'elle devrait trouver à partir de ces données. Pour la transcription de la parole par exemple, il faut fournir au système des centaines d'heures (à minima) d'enregistrements transcrits par des humains pour qu'il puisse apprendre comment produire lui-même ce type de transcription sur des données qu'il n'aura jamais vues.

En fonction du logiciel, ces données peuvent être de plusieurs natures :

- Des __fichiers textes alignés__ en langue source / langue cible pour la traduction de la parole
- Des __images annotées__ avec ce qu'elles contiennent pour la reconnaissance d'image
- Des __fichiers audio/vidéo transcrits__ pour la reconnaissance de la parole
- Vous voyez le principe ?

Lorsque ces ressources sont accessibles, elles ne le sont généralement pas librement. Les mondes de l'audio, de la vidéo, de l'image et même du texte sont rongés par le copyright, les fameux : « __touche pas à mon travail ou je me fâche tout rouge__ » ou encore « __prems et pas toi nananèèèère !__ ».

En gros, seuls ceux qui peuvent payer ont le droit d'utiliser ces données. Ça exclut généralement le monde du logiciel libre où la plupart du travail est bénévole.

Mais dans le coup, __on risque d'avoir un sérieux problème non__ ? D'un côté on a des logiciels libres qui __contribuent depuis des décennies au bien commun__ et de l'autre côté __des données indispensables__ pour qu'ils fonctionnent, mais qui ne sont __pas disponibles librement__.


<div style="text-align:center;">
    <img alt="Schéma développement logiciel Open-Source données non libres" src="/images/schema_dev_opensource_donnees.png" />
</div>

## Un cas concrêt : reconnaissance de la parole en français

Prenons un cas concrêt que je connais bien de part mon parcours professionnel : la reconnaissance de la parole, et plus particulièrement la __reconnaissance de la parole en français__. Mais le principe est certainement généralisable à d'autres domaines similaires utilisant du « machine learning » et du « big data ».

Si actuellement vous souhaitez utiliser une solution libre de reconnaissance automatique de la parole performante en français pour transcrire vos audios/vidéos, c'est __impossible__ ([performante comme ça](http://demo.voxolab.com/an/)). Pas à cause d'un souci logiciel bien sûr, eux ils sont disponibles __depuis plus de dix ans librement__ (actuellement le plus utilisé est [Kaldi](), par le passé c'était [Sphinx]()).

Le souci est bien un souci de données. Pour apprendre un tel système, il faut des données, __beaucoup de données__ : de 300H à plus de 1000H transcrites à la main.

Nous pourrions partir du fait qu'avec une communauté open-source bien organisée, nous pourrions transcrire 300H d'audio à la main. Ça représente environ __2000H de travail avec des personnes très compétentes en français__, ce qui n'est quand même pas négligeable.

Quand bien même serait-il possible de transcrire ces 300H+ d'audio, il reste un souci : __la propriété des données__. Comme les images que vous trouvez sur internet, les vidéos et les audios que vous trouvez ne sont généralement __pas libres de droit__. Il est donc impossible d'apprendre un système de reconnaissance automatique de la parole avec.

Exit donc la plupart des vidéos Youtube, des podcast de radio, des émissions de télé. Ça fait qu'il n'en reste pas lourd.

## Et donc ?

Il est donc important de comprendre que le __nerf de la guerre, c'est maintenant les données__. C'est très vrai pour les entreprises, ça l'est encore plus pour le monde du logiciel libre. Les entreprises peuvent mettre les moyens, le monde du logiciel libre beaucoup moins.

Notre monde informatique actuel a été façonné grâce aux logiciels libres. Il serait dommage de __manquer le virage du monde informatique de demain__ en le laissant dans l'unique main d'entreprises ou d'organismes privés.

Je n'ai pas de réponse immédiate à ces problématiques, mais je pense qu'il est important que la communauté du logiciel libre dans son ensemble en soit consciente.

Peut-être pourrions-nous créer __ce que Framasoft est pour le logiciel, mais pour les données__ : une association/organisation qui s'assure que le monde du libre propose une alternative aux géants qui ont l'argent pour avoir des données à ne plus savoir qu'en faire. Aller plus loin que ce qui se fait sur l'OpenData en France actuellement en créant de la __donnée à destination des systèmes d'apprentissage automatique__.

Peut-être que l'__ANR__ (Agence Nationale de la Recherche) devrait forcer toutes les données qui sont financées par notre argent à être disponibles sous licence libre de droit ? Régulièrement, l'ANR orchestre des __campagnes d'évaluation des systèmes__, et il ne me semblerait pas idiot que les données qui en sont issues soient __mises à disposition du plus grand nombre__ (c'est le cas pour certaines mais pas pour toutes).

Peut-être contacter tous les laboratoires de recherche francophones des différents domaines et voir avec eux ce qu'ils pourraient mettre à disposition librement ?

Même si la plupart d'entre nous sont nés dans un monde où le __logiciel libre était quelque chose de normal__, ça n'a pas toujours été le cas, et ça risque de ne plus l'être si l'on n'y prête pas suffisamment attention.

__Intéressé pour en discuter ? N'hésitez pas à me contacter directement sur Twitter [@vjousse](https://twitter.com/vjousse) ou sur Mastodon [@vjousse](https://mastodon.social/@vjousse).__
