<?php require_once( 'editar/cms.php' ); ?>
<cms:template title='Libros' hidden='0' order='3' clonable='1' dynamic_folders='1'> 
	
	<cms:editable
		name='libros' label='Fichas de libros' type='group' order='01'
	/>
	<cms:editable
		name='imagen'
		label='Portada del libro'
		type='image'
		order='09'
		show_preview='1'
		preview_width='300'
		group='libros'
	/>
	<cms:editable
	      	name='pp_miniatura'
	      	label='Miniatura, para la bolsa de la compra'
	     	width='50'
	     	height='77'
	      	show_preview='1'
	     	assoc_field='imagen'
	     	type='thumbnail'
	      	order='10'
		group='libros'
  	/>
	<cms:editable
		name='slide'
		label='Imagen para la galería de inicio'
		type='image'
		order='11'
		show_preview='1'
		preview_width='300'
		width='530'
		height='350'
		crop='1'
		group='libros'
	/>
	<cms:editable
		name='subtitulo'
		label='Subtítulo, si lo hubiese'
		type='text'
		order='12'
		group='libros'
	/>
	<cms:editable
		name='autor'
		label='Autor del libro'
		type='text'
		order='13'
		group='libros'
	/>
	<cms:editable
		name='paginas'
		label='Número de páginas'
		type='text'
		order='20'
		group='libros'
	/>
	<cms:editable
		name='formato'
		label='Dimensiones del libro (00 x 00)'
		type='text'
		order='21'
		group='libros'
	/>
	<cms:editable
		name='isbn'
		label='ISBN'
		type='text'
		order='22'
		group='libros'
	/>
	<cms:editable
		name='precio'
		label='Precio'
		type='text'
		order='23'
		group='libros'
	/>
	<cms:editable
		name='sinopsis1'
		label='Descripción catálogo'
		desc='la que aparece en la vista de catálogo'
		order='30'
		type='richtext'
		group='libros'
	/>
	<!--validator='max_len=550'-->
	<cms:editable
		name='sinopsis2'
		label='Descripción larga'
		desc='la que aparece en la vista de libro'
		type='richtext'
		order='31'
		group='libros'
	/>
	<cms:editable
		name='ano'
		label='Año de publicación'
		type='text'
		order='40'
		group='libros'
	/>
	<cms:editable
		name='prensa'
		label='archivo de nota de prensa'
		type='file'
		order='41'
		group='libros'
	/>
	<cms:editable
		name='medios'
		label='¿Hay enlaces a medios relacionados con este libro?'
		type='radio'
		opt_values='Sí=1 | No=0 '
		opt_selected='0'
		order='42'
		group='libros'
	/>
	<cms:editable
		name='lanzamiento'
		label='Fecha de lanzamiento'
		type='text'
		order='43'
		group='libros'
	/>
	<cms:editable
		name='colorlanzamiento'
		label='el color del texto de fecha de lanzamiento, igual que la portada (#000000)'
		type='text'
		group='libros'
		order='44'>#f00000</cms:editable>
	
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
				<cms:embed 'blogindex.html' />
				<cms:embed 'panelesq.html' />
				<div id="centro">
										
					<div class="book">
						<div class="entrada">
						<a href="<cms:show k_page_link />"><h3><cms:show k_page_title /></h3></a>
						<cms:if subtitulo><h4><cms:show subtitulo /></h4></cms:if>
						<a class="spacing" href="<cms:reverse_related_pages 'libros' masterpage='autores.php'><cms:show k_page_link /></cms:reverse_related_pages>"><h5><cms:show autor /></h5></a>
						</div>
						<a href="<cms:show imagen />" data-lightbox="<cms:show k_page_title />" data-title="<i><cms:show k_page_title /></i> · <cms:show autor />"><img class="portadita esq" src="<cms:show imagen />"></img></a>
						<div class="pieportada">
							<ul>
							<cms:if k_page_foldertitle><li>Col. <a href="<cms:show k_page_folderlink />"><cms:show k_page_foldertitle /></a></li></cms:if>
							<cms:if paginas><li><cms:show paginas /> páginas</li></cms:if>
							<cms:if formato><li>Formato: <cms:show formato />.</li></cms:if>
							<cms:if isbn><li>ISBN: <cms:show isbn /> </li></cms:if>
							<cms:if precio><li>Precio: <cms:show precio />

							</li>
							</cms:if>
							</ul>						
						</div>
						<div class="desc">
						
						<cms:show sinopsis2 />
					

						<cms:reverse_related_pages 'libros' masterpage='autores.php'><div class="librautor"><cms:show descripcion /></div></cms:reverse_related_pages>
						</div>
						<cms:if prensa><a class="prensa" href="<cms:show prensa />" target="_blank">Nota de prensa</a></cms:if>
						<cms:if lanzamiento><span class="lanzamiento" style="color:<cms:show colorlanzamiento />"><cms:show lanzamiento /> en librerías</span></cms:if>
						<cms:if medios = '1'>						
						<div class="medios">
							<h4>Prensa</h4>
							<ul>
								<cms:reverse_related_pages 'enlaces_prensa' masterpage='prensa.php' orderby='orden, publish_date'>
								<li>
									<cms:if autor ><cms:show autor />: </cms:if><a href="<cms:if archivo><cms:show archivo /><cms:else /><cms:show enlace /></cms:if>" target="_blank">«<cms:show k_page_title />»</a><cms:if espacio >, <i><cms:show espacio /></i></cms:if><cms:if medio >, <cms:show medio /></cms:if><cms:if fecha >, <cms:show fecha /></cms:if>.<br /><br />
									<div class="cent cont">
									<cms:if foto><a href="<cms:show enlace />" target="_blank"><img src="<cms:show foto/>" alt="<cms:show k_page_title />"></a></cms:if>
									<cms:if video><cms:show video/></cms:if>
									</div>
								</li>	
								</cms:reverse_related_pages>						
							</ul>						
						</div>
						</cms:if>
						
					</div>
					
					
					<div class="clear"></div>
					<!--lo del blog
					<a href="<cms:show k_site_link/>blog.php"><h3 class="blogfin"></h3></a>
					<ul class="listin">
						<cms:pages masterpage='libros.php' limit='3' orderby='random' id="<cms:concat 'NOT ' k_page_id />">
											
						<li class="listar <cms:zebra 'uno' 'dos' 'tres'/>">
							<a href="<cms:show k_page_link/>">
							<img class="foton indice anim" src="<cms:show miniaturaindex/>" alt="">
							<img class="foton mobil anim" src="<cms:show miniaturamovil/>" alt="">
							<p class="fecha anim"><cms:date k_page_date locale='spanish' format='%e|%m|%y'/></p>
							<div class="cacho">
							<h1 class="gancho anim"><cms:show k_page_title/></h1>
							</div>
							</a>
						</li>		
									
						</cms:pages>					
					</ul>
					
					<div class="clear"></div>
					-->
					<!--<cms:if k_page_folderpagecount gt '1' >
					<div class="related">
						<h4>De la misma colección</h4>
						<ul>
							<cms:pages masterpage='libros.php' folder=k_page_foldername orderby='random' id="<cms:concat 'NOT ' k_page_id />">
							<li><a href="<cms:show k_page_link/>">
								<img class="anim" src="<cms:show imagen />" alt="">
								<h5 class="tit anim"><cms:show k_page_title /></h5>
								<h5 class="aut spacing anim"><cms:show autor /></h5>
							</a></li>	
							</cms:pages>					
						</ul>		
					</div>
					</cms:if>-->
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
	<cms:embed 'catalogo.html' />
</cms:if>
<?php COUCH::invoke(); ?>
