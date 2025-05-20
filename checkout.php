<?php require_once 'editar/cms.php'; ?>
<cms:no_cache />
<cms:template title='Formas de pago y condiciones' hidden='0' order='9'>
	<cms:editable
		name='condiciones'
		label='Condiciones de venta, título en h4 y los tipos en forma de lista'
		type='richtext'
	/>
	<cms:editable
		name='transferencia'
		label='Explicación de como hacer ingreso, con número de cuenta'
		type='richtext'
	/>
	<cms:editable
		name='tarjetita'
		label='Explicación de como comprar con tarjeta'
		type='richtext'
	/>
	<cms:editable type='group' name='mail1' label='Mail que recibes tú' />
	<cms:editable
		name='encabezado'
		label='Encabezado antes de listar los datos'
		desc='no metas mucha virguería que solo puede ir texto plano sin negritas, ni subrayados...'
		type='richtext'
		group='mail1'
	/>
	<cms:editable
		name='despedida'
		label='Despedida después de listar los datos'
		desc='no metas mucha virguería que solo puede ir texto plano sin negritas, ni subrayados...'
		type='richtext'
		group='mail1'
	/>
	<cms:editable type='group' name='mail2' label='Mail que reciben ellxs' />
	<cms:editable
		name='encabezado2'
		label='Encabezado antes de listar los datos'
		desc='no metas mucha virguería que solo puede ir texto plano sin negritas, ni subrayados...'
		type='richtext'
		group='mail2'
	/>
	<cms:editable
		name='despedida2'
		label='Despedida después de listar los datos'
		desc='no metas mucha virguería que solo puede ir texto plano sin negritas, ni subrayados...'
		type='richtext'
		group='mail2'
	/>

</cms:template>
<!doctype html>
<html>
<head>
	<meta charset="utf-8">
        <meta http-equiv="X-UA-Compatible" content="IE=edge">
        <title>Díaz & Pons · Caja</title>
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
			<a class="navis" href="<cms:pp_cart_link />">
				<img class="rsn anim" src="<cms:show k_site_link/>img/prevn.png" alt="">
				<img class="rsr anim" src="<cms:show k_site_link/>img/prevr.png" alt="">
				<span class="rsname anim">Volver</span>
			</a>
			</div>
			</cms:if>
			</cms:pp_cart_form>

         <cms:if "<cms:pp_count_items />" >
	         <!-- START CHOOSE SHIPPING LOCATION FORM -->
	         <div class="destino">
	             <cms:form method="post" anchor='0'>
	                 <cms:if k_success >
	                     <cms:set_session name='selected_shipping_location' value=frm_shipping_location />
	                     <cms:pp_refresh_cart />
	                     <cms:redirect k_page_link />
	                 </cms:if>
	                 <h4>Elige una opción de destino</h4>
	                 <div class="styled">
	                 <cms:input type="dropdown"
	                     name="shipping_location"
	                     opt_values="
	                         ··· = 0 |
	                         Territorio nacional = 1 |
	                         Resto De Europa = 2 |
	                         Resto Del Mundo = 3 "
	                     opt_selected = "<cms:get_session 'selected_shipping_location' />"
	                     onchange="this.form.submit()"
	                     class="anim"
	                 />
	                 </div>
	             </cms:form>
	         </div>
	         <!-- END CHOOSE SHIPPING LOCATION FORM -->
			   <div class="destino">
			   <h4>Elige una forma de pago</h4>

			   <div class="transf">
			   	<span class="button anim">Transferencia</span>
			   </div>
	           <cms:form method="post" anchor='0'>
	               <cms:if k_success >
	                   <cms:pp_payment_gateway
	                       shipping_address="<cms:if "<cms:pp_count_shippable_items />" >1<cms:else />0</cms:if>"
	                       empty_cart='0'
	                   />
	                   <cms:pp_empty_cart />
	               </cms:if>

	               <div class="paypal">
	                   <span class="button anim" ><cms:input name="paypal" class="anim" type="submit" value="PayPal" /></span>
	               </div>
	           </cms:form>

	           <div class="tarjeta">
			   	<span class="button anim">Tarjeta de Crédito</span>
			   </div>
           	</div>

         <cms:else />
             <div class="message">
                 <p class="info">La bolsa está vacía.</p>
             </div>
         </cms:if>


			<cms:form method="post" enctype="multipart/form-data" class="formdirec">

			<cms:if k_success >
					<cms:send_mail from='info@diazpons.com' to='info@diazpons.com' subject="Pedido">
<cms:show encabezado/>

Datos
 · <cms:show k_success />

Pedido
<cms:pp_cart_items>
 · <cms:show title/> (<cms:number_format price />€) x <cms:show quantity /> = <cms:number_format line_total />€
</cms:pp_cart_items>
 · Gastos de envío = <cms:number_format "<cms:pp_shipping />" />€

 · Total = <cms:number_format "<cms:pp_total />" />€

<cms:show despedida/>
					</cms:send_mail>
					<cms:send_mail from='info@diazpons.com' to=frm_email subject="Díaz&Pons · Pedido confirmado">
<cms:show encabezado2/>

Datos
 · <cms:show k_success />

Pedido
<cms:pp_cart_items>
 · <cms:show title/> (<cms:number_format price />€) x <cms:show quantity /> = <cms:number_format line_total />€
</cms:pp_cart_items>
 · Gastos de envío = <cms:number_format "<cms:pp_shipping />" />€

 · Total = <cms:number_format "<cms:pp_total />" />€

<cms:show despedida2/>
					</cms:send_mail>
					<cms:pp_empty_cart />
					<cms:redirect url='http://www.diazpons.com/gracias.php' />

								<div class="hecho">
				        <h5>Enviado.</h5>
				        		</div>
				    </cms:if>

				    <cms:if k_error >
				    	<div class="hecho">
				        <h5>Algo falla. Prueba otra vez.</h5>
				        <cms:each k_error >
				            <cms:show item /><br>
				        </cms:each>
				        <h5><a href="<cms:show k_site_link/>checkout.php" >Corrige el campo sugerido, o pulsa aquí para recargar el formulario en blanco</a></h5>
				      </div>

				    </cms:if>
				   <div class="ingreso">
						<cms:show transferencia/>
				   </div>
					<h4>Dirección:</h4>
					<ul>
				    <li><h5>Nombre</h5>
					<cms:input class="cadra"
						type="text"
						size="10"
						maxlength="40"
				    	required="1"
						name="nombre"/></li>
				    <li><h5>Apellidos</h5>
				    <cms:input class="cadra"
				    	type="text"
				    	size="10"
				    	required="1"
				    	name="apellidos" /></li>
				    <li><h5>Calle y número</h5>
				    <cms:input class="cadra"
				 		 type="text"
				 		 size="10"
				    	required="1"
				 		 name="calle" /></li>
				    <li><h5>Localidad</h5>
				    <cms:input class="cadra"
				    	type='text'
				    	required="1"
				    	name="localidad" /></li>
				    <li><h5>C.P.</h5>
				    	<cms:input class="cadra"
				    	type="text"
				    	required="1"
				    	name="cp" /></li>
				   <li><h5>Email</h5>
				    <cms:input class="cadra"
				    	type="text"
				    	size="10"
				    	name="email"
				    	validator="email"
				    	validator_msg="required=Pon un mail válido"
				    	required="1" /></li>
				   <li><h5>Teléfono de contacto</h5>
			    	<cms:input class="cadra cadra2"
			    		type="text"
			    		size="10"
			    		validator='min_len=9'
			    		required='1'
			    		validator_msg="El teléfono ha de tener 9 cifras como mínimo"
			    		name="telefono" /></li>


			    	<cms:input class="codro anim"
			    		name="submit"
			    		type="submit"
			    		value="Enviar" />
			    	 </ul>
				</cms:form>


				<div class="credito">
				<cms:show tarjetita/>
				</div>

			<div class="clear"></div>
			<div class="condiciones rel">
			<h3 class="pinx">Condiciones de venta <span class="max">+</span><span class="menox">&nbsp;-</span></h3>
			<div class="condix">
			<cms:show condiciones/>
			</div>
			</div>
			<div class="clear"></div>

			<!--<cms:dump_all/>-->

			<div class="arriba"><a href="#arriba"><img src="<cms:show k_site_link/>img/arriba.png" alt="subir" title="Arriba"></img></a></div>
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

         <script>
				var direc = function () {
					$(".transf").click(function () {
						$(".formdirec").slideToggle();
						$(".credito").hide();
						$(".transf .button").addClass("roxo");
						$(".tarjeta .button").removeClass("roxo");
					});
				}
				(document).ready(direc);
        </script>
         <script>
				var tarje = function () {
					$(".tarjeta").click(function () {
						$(".credito").slideToggle();
						$(".formdirec").hide();
						$(".tarjeta .button").addClass("roxo");
						$(".transf .button").removeClass("roxo");
					});
				}
				(document).ready(tarje);
        </script>
         <script>
				var paypa = function () {
					$(".paypal").click(function () {
						$(".credito").hide();
						$(".formdirec").hide();
						$(".transfe .button").removeClass("roxo");
						$(".tarjeta .button").removeClass("roxo");
						$(".paypal").addClass("roxo");
					});
				}
				(document).ready(paypa);
        </script>
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
						$(".condix").slideToggle();
						$(".max").toggle();
						$(".menox").toggle();
					});
				}
				(document).ready(grella);
        </script>
</body>
</html>
<?php COUCH::invoke(); ?>