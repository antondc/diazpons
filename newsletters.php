<?php require_once( 'editar/cms.php' ); ?>
<cms:template title='Newsletters' hidden='0' order='91' clonable='1' executable='1' >

	<cms:editable
		name='enlacetitulo'
		label='enlacetitulo'
		order='01'
		type='text'
	/>
	<cms:editable
		name='subtitulo'
		label='subtitulo'
		order='02'
		type='text'
	/>
	<cms:editable
		name='autor'
		label='autor'
		order='03'
		type='text'
	/>
	<cms:editable
		name='colorportada'
		label='color de portada'
		type='text'
		group='libros'
		order='04'>#f00000</cms:editable>
	<cms:editable
		name='imagen'
		label='imagen'
		order='05'
		type='image'
	/>
	<cms:editable
		name='enlaceimagen'
		label='Enlace de la imagen'
		order='06'
		type='text'
	/>
	<cms:editable
		name='coleccion'
		label='coleccion'
		order='07'
		type='text'
	/>
	<cms:editable
		name='enlacecoleccion'
		label='enlacecoleccion'
		order='08'
		type='text'
	/>
	<cms:editable
		name='paginas'
		label='paginas'
		order='09'
		type='text'
	/>
	<cms:editable
		name='formato'
		label='formato'
		order='10'
		type='text'
	/>
	<cms:editable
		name='ISBN'
		label='ISBN'
		order='11'
		type='text'
	/>
	<cms:editable
		name='precio'
		label='precio'
		order='12'
		type='text'
	/>
	<cms:editable
		name='contenido'
		label='contenido'
		order='13'
		type='richtext'
	/>
</cms:template>

<html>
<head>
    	<link rel="icon" type="image/png" href="http://diazpons.com/testando/corporate/favicon.png">

		<meta charset="utf-8">
        <meta http-equiv="X-UA-Compatible" content="IE=edge">
        <title>Díaz & Pons Newsletters</title>
        <meta name="description" content="">
        <meta name="viewport" content="width=device-width, initial-scale=1">
</head>
<body style="height: 100%;background:  #FFFFFA;width: 600px;margin-left: auto;margin-right: auto;height: 100%;">
<cms:embed 'explorer.html' />

<!-- // Si no ves bien este Newsletter haz click aquí \\ -->
<p style="font-family:Arial, Helvetica, sans-serif; font-size:11px; color:#555555; margin-bottom:5px; text-align:center;">Si no ves correctamente este newsletter haz clic<a href="<cms:show k_page_link />" style="color:<cms:show colorportada />;text-decoration:none;"> aqu&iacute;</a></p>
<!-- // Si no ves bien este Newsletter haz click aquí \\ -->


<!-- // Cuerpo \\ -->
<table width="550" cellpadding="0" cellspacing="0" align="center" style="font-family:Arial, Helvetica, sans-serif; font-size:13px; line-height:14px;">
<!-- // Cuerpo \\ -->


<!-- // Cabecera \\ -->
	<tr>
    	<td width="550" height="120" colspan="3" valign="top" style="padding-top:30px;padding-bottom: 40px;border:0px;">
    		<a href="http://www.diazpons.com"><img src="http://www.diazpons.com/img/cabecera.png" width="550" border="0px" alt="Díaz & Pons - Novedades febrero 2015" /></a>
    	</td>
  	</tr>
  	<!-- // Cabecera \\ -->

<!-- // Libro \\ -->
	<table width="550" align="center">
		<tr>
 			<th style: "position: relative;width: 180px;vertical-align: top;">
 				<a href="<cms:show enlaceimagen />"><img style="width: 260px;border:0px;height: auto;" src="<cms:show imagen />"></a>
 			</th>
 			<td style="position: relative;width: 430px;vertical-align: top;">
 				<div style="font-family: 'Oswald', sans-serif;font-weight: 700; letter-spacing: -0.05em; font-size: 25px;text-align: left;text-transform: uppercase;margin-top:-2px;padding-left: 4px;"><a style="color: #292e26; text-decoration:none;" href="<cms:show enlacetitulo />"><cms:show k_page_title /></a></div>
 				<div style="color:#292e26;font-family: 'Raleway', sans-serif;letter-spacing: -0.01em;font-size: 15px;font-weight: 300;text-align: left;line-height: 15px;padding-left: 5px;margin-top: 5px;"><cms:show subtitulo /></div>
 				<div style="color:#292e26;font-family: 'Raleway', sans-serif;font-size: 18px;font-weight: 700;text-align: left;padding-left: 5px;padding-bottom: 130px;margin-top: 12px;line-height: 1em;"><cms:show autor /></div><br><br><br>
 				<div style="color:#292e26;text-align: justify;line-height: 1.4em;padding-top: 65px;margin-left: 3px; font-family: 'arial', sans-serif;font-size: 12px;padding-bottom: 0px;font-weight: 400;">

							<div>Col. <a style="text-decoration:none; color: #000000; " href="<cms:show enlacecoleccion />"><cms:show coleccion /></a></div>
							<div><cms:show paginas /> páginas</div>
							<div>Formato: <cms:show formato /> cm.</div>
							<div>ISBN: <cms:show ISBN /></div>
							<div>Precio: <cms:show precio /> &#8364;<div>
						</div>
 			</td>
 		</tr>
 		<tr>
 			<td colspan="2">
 				<div style="color:#292e26;text-align: justify;line-height: 1.4em;padding-top: 30px;font-family: 'arial', sans-serif;font-size: 14px;padding-bottom: 50px;"><cms:show contenido /><br>


 				</div>
			</td>
 		</tr>
 	</table>
<!-- // Libro \\ -->

<!-- // Pie \\ -->
	<table align="center">
	<tr>
    	<td align="center" width="550" colspan="3">
    		<a target="_blank" href="https://www.facebook.com/diazponseditores?ref=hl" style="color: #292e26; text-decoration:none;"><img src="http://www.diazpons.com/img/fbb.png" style="border-style: none;"/>&nbsp;</a>
    		<a target="_blank" href="https://twitter.com/DiazyPons" style="color: #292e26; text-decoration: none">&nbsp;<img src="http://www.diazpons.com/img/twb.png" style="border-style: none;"/></a>
    	</td>
	</tr>
	<tr>
    	<td width="550" colspan="3">
    		<p style="font-family: 'arial';font-size:12px; color:#555555; margin-top:5px; margin-bottom:0px; text-align:center; text-decoration:none;">Distribución<br></p>


    		<p style="font-family: 'arial';font-size:12px; color:#555555; margin-top:3px; margin-bottom:0px; text-align:center; text-decoration:none;"><a href="http://www.udllibros.com/html/pro/index.php?fr_deltaMes=" style="color:<cms:show colorportada />;text-decoration:none;"> UDL Libros</a></p>

    		<p style="font-family: 'arial';font-size: 12px;text-align: center; margin-top:3px;padding-bottom:15px;color:#555555">0034 671 156 605 · <a href="http://www.diazpons.com" style="color:#555555;text-decoration:none;">www.diazpons.com</a> · <a href="mailto:info@diazpons.com" style="color:#555555;text-decoration:none;">info@diazpons.com</a></span></p>

    	</td>
	</tr>
	</table>
<!-- // Pie \\ -->

<!-- // Texto legal \\ -->

<table width="565" align="center">
    <tbody>
        <tr>
            <td colspan="2">
            <div style="color:#292e26;text-align: justify;line-height: 1.4em;padding-top: 0px;font-family: 'arial', sans-serif;font-size: 9px;padding-bottom: 50px;margin-left: 10px; margin-right: 10px;">
            <p>Si recibes esta comunicación es porque has contactado con Díaz & Pons o cumplimentado el formulario de suscripción, aceptando las Condiciones de uso y la Política de Privacidad y, consecuentemente, la recepción de comunicaciones relacionadas. Si lo deseas puedes darte de baja <a href="mailto:info@diazpons.com?Subject=Baja%20de%20suscripción&body=Se solicita baja de esta dirección de correo." target="_top" style="color: #3a3a3a; text-decoration: none;">escribiéndonos aquí</a>.</p>
            </div>
            </td>
        </tr>
    </tbody>
</table>

<!-- // Texto legal \\ -->

</table>
</body>
</html>


<?php COUCH::invoke(); ?>