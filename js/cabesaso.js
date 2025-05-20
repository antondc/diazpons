$(document).ready(function () {
	$('#esq h4').click(function () {
		$('.logo, .boton, .mailcab').animate({top: '-=42px'},'fast');
		});
	$('#esq ul li').click(function () {
		$('.logo, .boton, .mailcab').animate({top: '+=42px'},'fast');
		});
	});