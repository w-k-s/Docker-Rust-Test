(function(){
	'use strict';

	angular
	.module('app')
	.controller('LoginController',LoginController);

	LoginController.$inject = ['$location','AuthenticationService'];

	function LoginController($location,AuthenticationService){

		var vm = this;

		(function initController(){
			AuthenticationService.ClearCredentials();
		})();

		vm.login = function(){
			AuthenticationService.Login(vm.username, vm.password, function(response){
				console.log(response);
			});
		};


	}
})();