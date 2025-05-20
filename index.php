<?php require_once( 'editar/cms.php' ); ?>
<cms:template title='Portada' hidden='0' order='0'>
	<cms:editable
		name='cookie'
		label='Frase de advertecia sobre las cookies'
		type='richtext'
	/>
	<cms:editable
		name='acepto'
		label='Texto del boton de aceptar las cookies'
		type='text'
	/>
	
</cms:template>
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

        <cms:embed 'css.html' />
        
        <script src="<cms:show k_site_link/>js/vendor/modernizr-2.6.2.min.js"></script>
        <script src="<cms:show k_site_link/>js/jquery-1.11.0.min.js"></script>
        <script src="<cms:show k_site_link/>js/lightbox.min.js"></script>
        <!-- para slideshow -->
			<script type="text/javascript" src="<cms:show k_site_link/>js/unslider.js"></script>	
			  
    </head>
    <body>
        <cms:embed 'explorer.html' />
        <!-- Add your site or application content here -->
        
       
			<cms:embed 'cabaceira.html' />

			<div id="corpo" class="rel">
				<cms:embed 'blogindex.html' />
				<cms:embed 'panelesq.html' />
				<div id="centro">
					<!--<div class="slide">
						Galería molona
					</div>-->
					<!--<div id="galeria-principal" class="galeria">
						<ul id="lista-galeria">
							<li title="El primero" data-subtitulo="Su autor"><img src="img/submarinos/1.jpg" alt="[Texto alternativo imagen]" width="[Indicar tamaño real de imagen en px]" /></li>
							<li title="Titulo libro" data-subtitulo="Autor"><img src="img/submarinos/2.jpg" alt="[Texto alternativo imagen]" width="[Indicar tamaño real de imagen en px]" /></li>
							<li title="Titulo libro" data-subtitulo="Autor"><img src="img/submarinos/3.jpg" alt="[Texto alternativo imagen]" width="[Indicar tamaño real de imagen en px]" /></li>
							<li title="Titulo libro" data-subtitulo="Autor"><img src="img/submarinos/4.jpg" alt="[Texto alternativo imagen]" width="[Indicar tamaño real de imagen en px]" /></li>
							<li title="Titulo libro" data-subtitulo="Autor"><img src="img/submarinos/5.jpg" alt="[Texto alternativo imagen]" width="[Indicar tamaño real de imagen en px]" /></li>
							<li title="Titulo libro" data-subtitulo="Autor"><img src="img/submarinos/6.jpg" alt="[Texto alternativo imagen]" width="[Indicar tamaño real de imagen en px]" /></li>
							<li title="Titulo libro" data-subtitulo="Autor"><img src="img/submarinos/7.jpg" alt="[Texto alternativo imagen]" width="[Indicar tamaño real de imagen en px]" /></li>
						</ul>
						<div class="botones">
							<a href="#" id="left" class="boton">⟨</a>
							<a href="#" id="right" class="boton">⟩</a>
						</div>
					</div>-->
					<div id="Fader" class="fader">
						<cms:pages masterpage='libros.php' limit='8' orderby='random'>
						<cms:if slide>
						<a href="<cms:show k_page_link/>"><img class="slide" src="<cms:show slide/>" title="<cms:show k_page_title/>"></a>
						</cms:if>
						</cms:pages>
						<!--<div class="fader_controls">
							<!--<div class="page prev" data-target="prev">&lsaquo;</div>
							<div class="page next" data-target="next">&rsaquo;</div>
							<ul class="pager_list">
							</ul>
						</div>-->
					</div>
					<ul>
					<cms:query
					    sql="SELECT p.id, p.template_id
					        FROM <cms:php>echo K_TBL_PAGES;</cms:php> p
					        inner join <cms:php>echo K_TBL_TEMPLATES;</cms:php> t
					        on p.template_id = t.id
					        WHERE (t.name='libros.php' or t.name='noticias.php')
					        AND publish_date < '<cms:date format='Y-m-d H:i:s' />'
					        AND NOT publish_date = '0000-00-00 00:00:00'
					        ORDER BY publish_date desc;"
					    limit='3'
					    fetch_pages='1'
					    >

					    <li class="libro rel anim">
					    		<cms:if k_template_name='libros.php'>
					    		<div class="entrada">
								<a href="<cms:show k_page_link />"><h3><cms:show k_page_title /></h3></a>
								<h4><cms:show subtitulo /></h4>
								<a class="spacing" href="<cms:reverse_related_pages 'libros' masterpage='autores.php'><cms:show k_page_link /></cms:reverse_related_pages>"><h5><cms:show autor /></h5></a>
								</div>
							   <a href="<cms:show k_page_link />"><img class="portadita esq centur" src="<cms:show imagen />"></img></a>
								<div class="desc">
									<cms:show sinopsis1 />
									<cms:if lanzamiento><span class="proxim" style="color:<cms:show colorlanzamiento />"><cms:show lanzamiento/> en librerías.</span></cms:if>
								</div>

								</cms:if>
								<cms:if k_template_name='noticias.php'>
								<div class="entrada">
								<a href="<cms:show k_page_link />"><h3><cms:show k_page_title /></h3></a>
								<h4><cms:show subtitulo /></h4>
								</div>
								<a href="<cms:show k_page_link />"><img class="portadita esq centur" src="<cms:show miniaturaindice />"></img></a>
								<div class="desc"><cms:excerptHTML count='66' ignore='img'><cms:show contenido /></cms:excerptHTML></div>
							   </cms:if>
							</li>

					</cms:query>

					</ul>

					<div class="clear"></div>

					<div class="arriba"><a href="#arriba"><img src="<cms:show k_site_link/>img/arriba.png" alt="subir" title="Arriba"></img></a></div>
				</div>
				<cms:embed 'panelder.html' />
			</div>



			<cms:embed 'pe.html' />
			<!--<div class="cookie fix">
				<cms:show cookie />
				<span class="cookboton abs anim"><cms:show acepto/></span>
			</div>-->
			<cms:embed 'grella.html' />

			 

        <script src="//ajax.googleapis.com/ajax/libs/jquery/1.10.2/jquery.min.js"></script>
        <script>window.jQuery || document.write('<script src="<cms:show k_site_link/>js/vendor/jquery-1.10.2.min.js"><\/script>')</script>
        <script src="<cms:show k_site_link/>js/plugins.js"></script>
        <script src="<cms:show k_site_link/>js/main.js"></script>

        <!-- Google Analytics: change UA-XXXXX-X to be your site's ID. -->
        <script>
            (function(b,o,i,l,e,r){b.GoogleAnalyticsObject=l;b[l]||(b[l]=
            function(){(b[l].q=b[l].q||[]).push(arguments)});b[l].l=+new Date;
            e=o.createElement(i);r=o.getElementsByTagName(i)[0];
            e.src='//www.google-analytics.com/analytics.js';
            r.parentNode.insertBefore(e,r)}(window,document,'script','ga'));
            ga('create','UA-XXXXX-X');ga('send','pageview');
        </script>
         <cms:embed 'javas.html' />
        
    </body>
</html>
<?php COUCH::invoke(); ?>
