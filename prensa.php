<?php require_once( 'editar/cms.php' ); ?>
<cms:template title='Prensa' hidden='0' order='6' clonable='1' executable='0' > 
	<cms:editable
		name='autor'
		label='Autor del artículo'
		order='01'
		type='text'
	/>
	<cms:editable
		name='espacio'
		label='Espacio'
		order='01'
		type='text'
	/>
	<cms:editable
		name='medio'
		label='Medio que lo publica'
		order='02'
		type='text'
	/>
	<cms:editable
		name='fecha'
		label='Fecha en que se publica en formato xx de mes de 20xx'
		order='03'
		type='text'
	/>
	<cms:editable
		name='enlace'
		label='enlace al medio'
		order='04'
		type='text'
	/>
	<cms:editable
		name='archivo'
		label='archivo a enlazar'
		order='05'
		type='file'
	/>
	<cms:editable
		name='enlaces_prensa'
		masterpage='libros.php'
		type='relation'
		order='06'
		label='Libro al que hace referencia'
	/>
	<cms:editable
		name='foto'
		label='imagen de prensa'
		order='07'
		type='image'
	/>
	<cms:editable
		name='video'
		label='codigo del video o plugin a enlazar'
		order='08'
		type='textarea'
		no_xss_check='1' 
	/>
	<cms:editable
		name='orden'
		label='Un número para el orden en el listado, por defecto ordena por fecha'
		order='09'
		type='text'
	/>
		
	
	
</cms:template>

<html>
<head>
</head>
<body>
<cms:dump_all/>
</body>		
</html>


<?php COUCH::invoke(); ?>