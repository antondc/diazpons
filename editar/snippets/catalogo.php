<?php require_once( 'editar/cms.php' ); ?>
<cms:template title='Portada' hidden='0' order='0' />
<!DOCTYPE html>
<!--[if lt IE 7]>      <html class="no-js lt-ie9 lt-ie8 lt-ie7"> <![endif]-->
<!--[if IE 7]>         <html class="no-js lt-ie9 lt-ie8"> <![endif]-->
<!--[if IE 8]>         <html class="no-js lt-ie9"> <![endif]-->
<!--[if gt IE 8]><!--> <html class="no-js"> <!--<![endif]-->
    <head>
        <meta charset="utf-8">
        <meta http-equiv="X-UA-Compatible" content="IE=edge">
        <title>Díaz & Pons</title>
        <meta name="description" content="">
        <meta name="viewport" content="width=device-width, initial-scale=1">

        <!-- Place favicon.ico and apple-touch-icon.png in the root directory -->

        <link rel="stylesheet" href="css/normalize.css">
        <link rel="stylesheet" href="css/main.css">
        <link rel="stylesheet" href="css/lightbox.css">
		  <link rel="stylesheet" href="css/unslider.css" type="text/css" media="screen" />
        <script src="js/vendor/modernizr-2.6.2.min.js"></script>
        <script src="js/jquery-1.11.0.min.js"></script>
        <script src="js/lightbox.min.js"></script>
        <script type="text/javascript" src="js/unslider.js"></script>	
    </head>
    <body>
        <!--[if lt IE 7]>
            <p class="browsehappy">Está usando un navegador <strong>obsoleto</strong>. Por favor <a href="http://browsehappy.com/">actualicé a la última versión</a> a la voz de <strong>ya</strong>.</p>
        <![endif]-->
        <!--[if IE 8]>
			<script src="js/IE9.js"></script>
			<script src="js/css3-mediaqueries.js"></script> 
			 <![endif]-->

        <!-- Add your site or application content here -->
        
       
			<cms:embed 'cabaceira.html' />
			
			<div id="corpo" class="rel">
				<cms:embed 'panelesq.html' />
				<div id="centro">
					<cms:pages masterpage='libros.php' limit='5'>
					<div class="libro rel anim">
						<div class="entrada">
						<a href="<cms:show k_page_link />"><h3><cms:show k_page_title /></h3></a>
						<p><i><cms:show subtitulo /></i></p>
						<a class="spacing" href="#"><p><cms:show autor /></p></a>
						</div>
						<a href="<cms:show k_page_link />"><img class="portadita esq" src="<cms:show imagen />"></img></a>
						<p class="desc"><cms:show sinopsis /></p>
						<cms:if lanzamiento><span class="proxim abs" style="color:<cms:show colorlanzamiento />"><cms:show lanzamiento/> en librerías.</span></cms:if>
					</div>
					</cms:pages>
					
					
					
					<div class="arriba"><a href="#arriba"><img src="img/arriba.png" alt="subir" title="Arriba"></img></a></div>
				</div>
				<cms:embed 'panelder.html' />
			</div>
			
			<cms:embed 'pe.html' />
			<cms:embed 'grella.html' />
			
			 

        <script src="//ajax.googleapis.com/ajax/libs/jquery/1.10.2/jquery.min.js"></script>
        <script>window.jQuery || document.write('<script src="js/vendor/jquery-1.10.2.min.js"><\/script>')</script>
        <script src="js/plugins.js"></script>
        <script src="js/main.js"></script>

        <!-- Google Analytics: change UA-XXXXX-X to be your site's ID. -->
        <script>
            (function(b,o,i,l,e,r){b.GoogleAnalyticsObject=l;b[l]||(b[l]=
            function(){(b[l].q=b[l].q||[]).push(arguments)});b[l].l=+new Date;
            e=o.createElement(i);r=o.getElementsByTagName(i)[0];
            e.src='//www.google-analytics.com/analytics.js';
            r.parentNode.insertBefore(e,r)}(window,document,'script','ga'));
            ga('create','UA-XXXXX-X');ga('send','pageview');
        </script>
        <script type="text/javascript" src="js/scrollAnchor.js"></script>
        <script type="text/javascript" src="js/cabesa.js"></script>
        <script>
				var grella = function () {
					$("#pe").click(function () {
						$("#grella").toggle(500);
					});
				}  
				(document).ready(grella);      
        </script>
        <script type="text/javascript"> setInterval(irderecha, 5000); </script> 
        
    </body>
</html>
<?php COUCH::invoke(); ?>