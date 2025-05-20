<?php require_once( 'editar/cms.php' ); ?>
<cms:template title='Autores' hidden='0' order='5' clonable='1'> 
	<cms:editable
		name='imagen'
		label='Foto del autor'
		type='image'
	/>
	<cms:editable
		name='descripcion'
		label='Breve descripción del autor'
		type='richtext'
	/>
	<cms:editable 
		type='relation' 
		name='libros' 
		masterpage='libros.php' />
	
</cms:template>

<cms:if k_is_page >
<!DOCTYPE html>
<!--[if lt IE 7]>      <html class="no-js lt-ie9 lt-ie8 lt-ie7"> <![endif]-->
<!--[if IE 7]>         <html class="no-js lt-ie9 lt-ie8"> <![endif]-->
<!--[if IE 8]>         <html class="no-js lt-ie9"> <![endif]-->
<!--[if gt IE 8]><!--> <html class="no-js"> <!--<![endif]-->
    <head>
        <meta charset="utf-8">
        <meta http-equiv="X-UA-Compatible" content="IE=edge">
        <title>Díaz & Pons · <cms:show k_template_title /></title>
        <meta name="description" content="">
        <meta name="viewport" content="width=device-width, initial-scale=1">

        <!-- Place favicon.ico and apple-touch-icon.png in the root directory -->

        <cms:embed 'css.html' />
        <script src="<cms:show k_site_link/>js/vendor/modernizr-2.6.2.min.js"></script>
        <script src="<cms:show k_site_link/>js/jquery-1.11.0.min.js"></script>
        <script src="<cms:show k_site_link/>js/lightbox.min.js"></script>
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
					<div class="book">
						<div class="entrada">
						<a href="<cms:show k_page_link />"><h3><cms:show k_page_title /></h3></a>
						<!--<p><i><a href="#">blog</a></i></p>-->
						</div>
						<a href="<cms:show imagen />" data-lightbox="<cms:show k_page_name />" data-title="<cms:show k_page_title />"><img class="portadita autor esq" src="<cms:show imagen />"></img></a>
						
						<div class="desc">
						<p><cms:show descripcion /></p>
						</div>
						<ul class="obras">Obras en Díaz & Pons
							<cms:related_pages 'libros' >
							<li><a href="<cms:show k_page_link />"><i><cms:show k_page_title /></i>, Díaz & Pons, Madrid, <cms:show ano />.</a></li>
							</cms:related_pages>
						</ul>
					</div>
					
					<!--<cms:dump_all/>-->
					
					<div class="arriba"><a href="#arriba"><img src="<cms:show k_site_link/>img/arriba.png" alt="subir" title="Arriba"></img></a></div>
				</div>
				<cms:embed 'panelder.html' />
			</div>
				<cms:embed 'slidelateral.html' />
				<cms:embed 'pe.html' />
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
<cms:else />
	<cms:embed 'elenco.html' />
</cms:if>
<?php COUCH::invoke(); ?>