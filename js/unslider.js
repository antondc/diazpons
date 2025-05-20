/*	
	UNSLIDER por Andrés Moreno
	Efecto que aparece en la página http://www.unslider.com
	sin jquery y creando las animaciones con CSS3 y esta librería Javascript
	Licencia Creative Commons
*/
var global = 0;
var limite = 0;
var comienzo = 0;
var fin = 0;
var galeria = null;
var listagaleria = null;
// Crear elementos
var crearElemento = function(tipoelem){
	return document.createElement(tipoelem);
}
// obtener el alto de la ventana
var altoVentana = function(){
	// le resto 10 por el border de 5px (superior e inferior)
	if (window.innerHeight != undefined)
		galeria.style.maxHeight = (window.innerHeight - 10)+'px';
	else
		galeria.style.maxHeight = (document.body.clientHeight - 10)+'px';
}
// Desplazar a derecha o izquierda
var irderecha = function(){
	if (global >= limite){global = 0;}
	else{global += 100;}	
	listagaleria.style.left = '-' + global + '%';
	cambiarBotonRedondo();
	altoVentana();
}
var irizquierda = function(){
	if (global <= 0){global = limite;}
	else{global -= 100;}	
	listagaleria.style.left = '-' + global + '%';
	cambiarBotonRedondo();	
	altoVentana();
}
// función para los botones de navegación
var navegacion = function(){
	switch (this.id){
		case 'right':
			irderecha();			
			break;
		case 'left':
			irizquierda();
			break;
	}
	return false;
}
//función para manejar con teclado
var navegacionteclado = function(elEvento){
	//alert(window.event);
	var prev = window.event || elEvento;
	switch (prev.keyCode){
		case 39:
			irderecha();
			break;
		case 37:
			irizquierda();
			break;
		case 'default':
			return false;
			break;
	}
	return true;
}

//función para manejar con el wheel del raton
var navegacionraton = function(elEvento){
	var prev = window.event || elEvento;
	var evento = elEvento.detail || elEvento.wheelDelta;
	switch (evento){
		// opera
		case 2:			
			irderecha();
			break;
		case -2:
			irizquierda();
			break;
		// firefox
		case 3:
			irderecha();
			break;
		case -3:
			irizquierda();
			break;
		//chrome & chromium
		case -120:
			irderecha();
			break;
		case 120:
			irizquierda();
			break;
	}
	// Deshabilita comportamiento por defecto raton al situarnos sobre la galeria
	if (prev.preventDefault)
		prev.preventDefault();
	return false;
}
// navegacion movil
var starttouch = function(elEvento){comienzo = elEvento.touches[0].pageX;}

var endtouch = function(elEvento){return null;}

var navegaciontouchmove = function(elEvento){
	fin = elEvento.touches[0].pageX;
	if (comienzo > fin)	irderecha();
	else if (comienzo < fin) irizquierda();
}
// navegacion indicadores
var crearIndicadores = function(num, idgaleria){			
	var c = crearElemento('div');
	c.id = 'indicadores'
	idgaleria.appendChild(c);
	var r = [];
	for (var i = 0; i < num; i++){
		r[i] = crearElemento('a');
		r[i].href = '#';
		r[i].id = i;
		r[i].className = 'boton-redondo';
		c.appendChild(r[i]);
	}
	r[0].className = 'boton-redondo selected';
	navegarIndicadores(c.id);
	return false;
}
var navegarIndicadores = function(id){
	var ind = document.getElementById(id).querySelectorAll('.boton-redondo');
	for (var i = 0; i < ind.length; i++){
		ind[i].onclick = (function(){
			listagaleria.style.left = '-' + (this.id * 100) + '%';
			global = (this.id * 100);
			for (var i = 0; i < ind.length; i++){
				ind[i].className = 'boton-redondo';
			}				
			this.className = 'boton-redondo selected';
			imagenVisible(this.id);
		});
	}
	return false;
}
// espia boton redondo
var cambiarBotonRedondo = function(){
	var posicion  = listagaleria.style.left,
		ind = document.getElementById('indicadores').querySelectorAll('.boton-redondo');
	for (var i = 0; i < ind.length; i++){ 
		ind[i].className = 'boton-redondo';
	}
	if (posicion.charAt(0) == '0') { 
		document.getElementById(posicion.charAt(0)).className = 'boton-redondo selected';
		imagenVisible(0);
	}
	else { 
		document.getElementById(posicion.charAt(1)).className = 'boton-redondo selected';
		imagenVisible(posicion.charAt(1));
	}
}
// imagen visible
var imagenVisible = function(num){
	var li = listagaleria.getElementsByTagName('li');
	for (var i = 0; i < li.length; i++){
		li[i].className = '';
	}
	li[num].className = 'visible';
}
// inicializar slide
var iniciar = function(){
	// variables locales:
	var botones = document.querySelectorAll('.boton'),
		len, i,
		totallimite = document.getElementById('lista-galeria').getElementsByTagName('li'),
		mousewheelevt = ((/Firefox/i.test(navigator.userAgent))? "DOMMouseScroll" : "mousewheel");	
	// Inicializar variables globales: 
	// NOTA: var 'limite' declarada como global (que luego se me olvida)
	// NOTA: En android browser tengo que iniciar las variables globales 'galeria' y 'listagaleria' aquí, ya que 
	// de otro modo no funciona el slide correctamente en el móvil;
	galeria = document.getElementById('galeria-principal');
	listagaleria = document.getElementById('lista-galeria');
	limite = (totallimite.length * 100) - 100; 
	// Iniciar estilos listagaleria:
	listagaleria.style.left = '0';
	listagaleria.style.width = (totallimite.length * 100) + '%';
	// Asignar el ancho a cada elemento li de la galería
	for (i = 0, len = totallimite.length; i < len; i++){
		totallimite[i].style.width = (100/len) + '%';
	}	
	totallimite[0].className = 'visible';
	// Manejadores de eventos
	for (i = 0, len = botones.length; i < len; i++){
		if (botones[i].addEventListener){
			botones[i].addEventListener('click',navegacion, false);
		}
		else {
			botones[i].onclick = navegacion;
		}
	}
	if (document.addEventListener){
		document.addEventListener('keydown', navegacionteclado, false);
	}
	else {
		document.attachEvent('onkeydown', navegacionteclado);
	}
	if (galeria.attachEvent) { //if IE (and Opera depending on user setting)
		galeria.attachEvent("onmousewheel", navegacionraton); 
	}	
	else if (galeria.addEventListener) {//WC3 browsers
		galeria.addEventListener(mousewheelevt, navegacionraton, false);
		galeria.addEventListener('touchstart', starttouch, false);
		galeria.addEventListener('touchend', endtouch, false);
		galeria.addEventListener('touchmove', navegaciontouchmove, false);
	}
	// Calcular el alto de la ventana para asignar un max-height al id galeria-principal
	altoVentana();
	// Crear los indicadores de navegacion
	crearIndicadores(totallimite.length, galeria);
}
window.onload = function(){
	iniciar();
}