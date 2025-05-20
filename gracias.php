<?php require_once 'editar/cms.php'; ?>
<cms:no_cache />
<cms:template title='Gracias' hidden='0' order='99999'> 
	<cms:editable
		name='gracias'
		label='Mensaje de agradecimiento por realizar la compra por transferencia'
		type='richtext'
	/>
	
</cms:template>
<!doctype html>
<html>
<head>
	<meta charset="utf-8">
        <meta http-equiv="X-UA-Compatible" content="IE=edge">
        <title>Díaz & Pons Libros · <cms:show k_template_title /></title>
        <meta name="description" content="">
        <meta name="viewport" content="width=device-width, initial-scale=1">
		 

        <!-- Place favicon.ico and apple-touch-icon.png in the root directory -->

        <cms:embed 'css.html' />
        <script src="<cms:show k_site_link/>js/vendor/modernizr-2.6.2.min.js"></script>
        <script src="<cms:show k_site_link/>js/jquery-1.11.0.min.js"></script>
        <script src="<cms:show k_site_link/>js/lightbox.min.js"></script>
        <script type="text/javascript" src="<cms:show k_site_link/>js/unslider.js"></script>
	<!--[if lt IE 9]>
		<script src="js/ie9.js" type="text/javascript"></script>
	<![endif]-->
</head>
<body>
	<cms:embed 'explorer.html' />
	<cms:embed 'cabaceira.html' />
	<div id="corpo" class="rel">
		<cms:embed 'blogindex.html' />
		<cms:embed 'panelesq.html' />
		<div id="centro" class="rel">
			<cms:pp_cart_form>
			<cms:if "<cms:pp_count_items />" >
			<div class="cesto">
			<table>
				<tr class="encab">
					<td class="dix"></td>					
					<td class="fif">pedido</td>						
				</tr>					
				<tr class="concepto">
					<td class="dix"> </td>						
					<td class="fif">título</td>						
					<td class="six">cantidad</td>						
					<td class="six">precio</td>						
					<td class="six">total</td>						
				</tr>
				<cms:pp_cart_items>
				<tr class="compra anim">
					<td class="dix"></td>
					<td class="fif"><cms:show title/></td>						
					<td class="six"><cms:show quantity /></td>						
					<td class="six"><cms:number_format price /><span style="font-family: FuturaMC;">€</span></td>						
					<td class="six"><cms:number_format line_total /><span style="font-family: FuturaMC;">€</span></td>						
				</tr>
				</cms:pp_cart_items>
				<cms:if "<cms:pp_shipping />">
				<tr class="envio envio">
					<td class="dix"> </td>					
					<td class="fif">Gastos de envío</td>					
					<td class="six"></td>					
					<td class="six"></td>					
					<td class="six"><cms:number_format "<cms:pp_shipping />" /><span style="font-family: FuturaMC;">€</span></td>					
				</tr>
				</cms:if>
				<tr class="total">
					<td class="dix"> </td>					
					<td class="fif">total</td>					
					<td class="six"><cms:pp_count_items /></td>					
					<td class="six"></td>					
					<td class="six"><cms:number_format "<cms:pp_total />" />€</td>					
				</tr>
			</table>
			
			</div>					
			</cms:if>					
			</cms:pp_cart_form>	
				
			<div class="gracias">
				<cms:show gracias/>			
			</div>         
         				
			<div class="clear"></div>
		
			<!--<cms:dump_all/>-->
			
		</div>
		<cms:embed 'panelder.html' />
	</div>
	<cms:embed 'slidelateral.html' />
	<cms:embed 'pe.html' />
	<cms:embed 'grella.html' />
	<script src="http://ajax.googleapis.com/ajax/libs/jquery/1.8.3/jquery.min.js" type="text/javascript"></script>
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
        <script type="text/javascript">
        		var condi = function () {
					$(".pinx").click(function () {
						$(".condix").toggle();
						$(".max").toggle();
						$(".menox").toggle();
					});
				}  
				(document).ready(grella); 
        </script>
</body>
</html>
<?php COUCH::invoke(); ?>