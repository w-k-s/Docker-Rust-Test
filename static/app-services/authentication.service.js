(function(){
	'use strict';

	angular
	.module('app')
	.factory('AuthenticationService',AuthenticationService)

	AuthenticationService.$inject = ['$http','$cookies','$rootScope',];
	function AuthenticationService($http,$cookies,$rootScope){

		var service = {};

		service.Login = Login;
		service.ClearCredentials = ClearCredentials;

		return service;

		function Login(username,password,callback){
			$http.post('api/login',{username: username, password: password})
			.then(function(success){
				console.log(success);
			}, function(error){
				console.log(error);
			});
		}

		function ClearCredentials(){
			console.log('Clear Credentials');
		}

		function setCredentials(){
			console.log('Set Credentials');
		}
	}

})();