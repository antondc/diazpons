<?php require_once( 'editar/cms.php' ); ?>
<cms:template title='Redes sociales' hidden='0' order='92' clonable='1' executable='0' > 
	<cms:editable
		name='link'
		label='Enlace de la red'
		type='text'
	/>
	<cms:editable
		name='negro'
		label='Icono en negro, de mínimo 16x16'
		type='image'
	/>
	<cms:editable
		name='rojo'
		label='Icono en rojo, de mínimo 16x16'
		type='image'
	/>
</cms:template>
<?php COUCH::invoke(); ?>