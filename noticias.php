<?php require_once( 'editar/cms.php' ); ?>
<cms:template title='Noticias' hidden='0' order='1' clonable='1' dynamic_folders='1'> 
	<cms:editable type='group' name='imagenes' label='imagenes de encabezado' order='10'/>	
	<cms:editable
		name='imagen'
		label='Imagen que encabeza el post'
		type='image'
		order='11'
		show_preview='1'
		width='530'
		preview_width='300'
		group='imagenes'
	/>
	<cms:editable
		name='miniaturalista'
		label='Miniatura, para indice de blog en pantalla grande'
		width='167'
		height='84'
		show_preview='1'
		assoc_field='imagen'
		type='thumbnail'
		order='12'
		group='imagenes'
	/>
  	<cms:editable
      name='miniaturamovil'
      label='Miniatura, para el indice en móvil'
      width='530'
      height='530'
      show_preview='1'
      assoc_field='imagen'
      type='thumbnail'
      order='13'
		group='imagenes'
  	/>
	<cms:editable
		name='miniaturaindice'
		label='Miniatura, para pagina inicial'
		width='116'
		height='180'
		show_preview='1'
		assoc_field='imagen'
		type='thumbnail'
		order='14'
		group='imagenes'
  	/>
	<cms:editable
		name='subtitulo'
		label='Subtítulo, si lo hubiese'
		type='text'
		order='01'
	/>
	
	<cms:editable
		name='contenido'
		label='El texto del post'
		order='30'
		type='richtext'
	/>
	<!--validator='max_len=550'-->
	
	<cms:editable
		name='frasearchivo'
		label='Texto que enlaza al archivo'
		desc='si la hubiese'
		type='text'
		order='35'
	/>
	<cms:editable
		name='archivo'
		label='Algún archivo que se quiera enlazar'
		type='file'
		order='34'
	/>
	<cms:editable
		name='videoarriba'
		label='Algún vídeo o plugin que encabece el post'
		type='textarea'
		no_xss_check='1' 
		order='40'
	/>
	<cms:editable
		name='videoabajo'
		label='Algún vídeo o plugin que cierre el post'
		type='textarea'
		no_xss_check='1'
		order='41' 
	/>
	
	
</cms:template>

<cms:if k_is_page>
<!DOCTYPE html>
<!--[if lt IE 7]>      <html class="no-js lt-ie9 lt-ie8 lt-ie7"> <![endif]-->
<!--[if IE 7]>         <html class="no-js lt-ie9 lt-ie8"> <![endif]-->
<!--[if IE 8]>         <html class="no-js lt-ie9"> <![endif]-->
<!--[if gt IE 8]><!--> <html class="no-js"> <!--<![endif]-->
    <head>
        <meta charset="utf-8">
        <meta http-equiv="X-UA-Compatible" content="IE=edge">
        <title>Díaz & Pons · <cms:show k_page_title /></title>
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
				<!--<cms:embed 'blogindex.html' />-->
				<cms:embed 'panelesq.html' />
				<div id="centro">
					
					<div class="bloc">
					<a href="<cms:show imagen />" data-lightbox="<cms:show k_page_title />" data-title="<cms:show k_page_title />"><img class="foton" src="<cms:show imagen/>" alt=""></a>
					<div class="cacho">
					<h1 class="gancho"><cms:show k_page_title/></h1>
					<cms:if subtitulopost><h3 class="subti"><cms:show subtitulopost/></h3></cms:if>
					<span class="fecha"><cms:date k_page_date locale='spanish' format='%e %m %G'/></span>&nbsp;&nbsp;&nbsp;
					<!--<span class="carp der"><a href="<cms:show k_page_folderlink/>">[<cms:show k_page_foldertitle/>]</a></span>-->
					</div>
					<ul class="datitos">
					<!--<cms:if autor><li class="quien">por <cms:show autor/></li></cms:if>-->
					</ul>
					<div class="clear"></div>
					<div class="corpinho">
					<div class="grueso">
					<cms:if videoarriba><cms:show videoarriba/></cms:if>
					<div class="lacosa"><cms:show contenido/></div>
					<cms:if videoabajo><cms:show videoabajo/></cms:if>
					</div>
					<!--<div class="fino">
					<cms:if frasaca><p class="frasaca">«<cms:show frasaca/>»</p></cms:if>
					<cms:if imagen1><div class="imagina">
					<a href="<cms:show imagen1 />" data-lightbox="<cms:show k_page_title />" data-title="<cms:show k_page_title />"><img class="delado" src="<cms:show imagen1/>" alt=""></a>
					<p class="piefoto"><cms:show pie1/></p>
					</div></cms:if>
					</div>-->
					</div>
					
					<div class="clear"></div>
					<!--
					<a href="<cms:show k_site_link/>blog.php"><h3 class="blogfin"></h3></a>
					<ul class="listin">
						<cms:pages masterpage='blog.php' limit='3' orderby='random' id="<cms:concat 'NOT ' k_page_id />">
						
						<li class="listar <cms:zebra 'uno' 'dos' 'tres'/>">
							<a href="<cms:show k_page_link/>">
							<img class="foton indice anim" src="<cms:show miniaturaindex/>" alt="">
							<img class="foton mobil anim" src="<cms:show miniaturamovil/>" alt="">
							<p class="fecha anim"><cms:date k_page_date locale='spanish' format='%e | %m | %y'/></p>
							<div class="cacho">
							<h1 class="gancho anim"><cms:show k_page_title/></h1>
							</div>
							</a>
						</li>		
						</cms:pages>					
					</ul>
					-->
					<div class="clear"></div>
					</div>
					
					
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
        <script src="http://ajax.googleapis.com/ajax/libs/jquery/1.11.0/jquery.min.js"></script> 
		  <script src="<cms:show k_site_link/>js/slideshow.js"></script>
        <script type="text/javascript" src="<cms:show k_site_link/>js/scrollAnchor.js"></script>
        
        <script type="text/javascript">
        		$(function(){
					$('#Fader').easyFader({
					slideDur: 6000,
					fadeDur: 800
					});
					});
        </script>
        <script type="text/javascript"> setInterval(irderecha, 3000); </script> 
        
    </body>
</html>
<cms:else />
<cms:embed 'lista-blog.html' />
</cms:if>
<?php COUCH::invoke(); ?>
