<!doctype html>
<html>
<head>
	<title>{{> title}}</title>
	{{> head_includes}}
	<script>
		$(document).ready(function(){
			$('#registrationForm').validate({
				errorClass: 'text-danger',
				rules: {
					first_name:{
						required: true,
						minlength: 2,
						maxlength: 45
					},
					last_name:{
						required: true,
						minlength: 2,
						maxlength: 45
					},
					email:{
						required: true,
						email: true,
						maxlength: 100
					},
					password:{
						required:true,
						regex: "/^(?=.*[a-z])(?=.*[A-Z])(?=.*\d)(?=.*[$@$!%*?&.])[A-Za-z\d$@$!%*?&.]{6, 20}/"
					},
					confirm_password:{
						required: true,
						equalTo: '#inputPassword'
					},
					terms_agreement:{
						required: '#inputTermsAgreement:checked'
					}
				},
				messages:{
					first_name:{
						required: "Please enter your first name",
						minlength: "Your first name must be at least 2 characters long",
						maxlength: "First name may not exceed 45 characters"
					},
					last_name:{
						required: "Please enter your last name",
						minlength: "Your last name must be at least 2 characters long",
						maxlength: "Last name may not exceed 45 characters",
					},
					email:{
						required: "Please enter your email",
						email: "Please enter a valid email address",
						maxlength: "Email address may not exceed 100 characters",
					},
					password:{
						required: "Please enter a password",
						regex: "Your password must be between 6 and 20 characters long with atleast one uppercase letter, one lower case letter and a special character"
					},
					confirm_password:{
						required: "Re-enter your password",
						equalTo: "Passwords do not match"
					},
					terms_agreement:{
						required: 'You must agree to the terms and conditions'
					}
				}
			});
		});
	</script>
</head>
<body>
{{> nav}}
<div class="jumbotron">
	<div class="container-fluid">
	  <div class="row">
	    <div class="col-lg-6 visible-md-block">.col-md-6</div>
	  	<div class="col-lg-6">
	  		<div>
				<form id="registrationForm" name="register" class="form-horizontal" method="post" action="/register">
				  <fieldset>
				    <legend>Sign up</legend>
				    <div class="form-group">
				      <label for="inputFirstName" class="col-lg-8 control-label">First Name</label>
				      <div class="col-lg-8">
				        <input type="text" class="form-control" id="inputFirstName" placeholder="First Name" name="first_name">
				      </div>
				    </div>
				    <div class="form-group">
				      <label for="inputLastName" class="col-lg-8 control-label">Last Name</label>
				      <div class="col-lg-8">
				        <input type="text" class="form-control" id="inputLastName" placeholder="Last Name" name="last_name">
				      </div>
				    </div>
				    <div class="form-group">
				      <label for="inputEmail" class="col-lg-8 control-label">Email</label>
				      <div class="col-lg-8">
				        <input type="email" class="form-control" id="inputEmail" placeholder="Email" name="email">
				      </div>
				    </div>
				    <div class="form-group ">
				      <label for="inputPassword" class="col-lg-8 control-label">Password</label>
				      <div class="col-lg-8">
				        <input type="password" class="form-control" id="inputPassword" placeholder="Password" name="password">
				      </div>
				    </div>
				    <div class="form-group ">
				      <label for="inputConfirmPassword" class="col-lg-8 control-label">Confirm Password</label>
				      <div class="col-lg-8">
				        <input type="password" class="form-control" name="confirm_password" id="inputConfirmPassword" placeholder="Confirm Password">
				      </div>
				    </div>
				    <div class="form-group ">
				      <div class="checkbox">
				          <label>
				            <input type="checkbox" class="form-control" name="terms_agreement" id="inputTermsAgreement">I agree to the <a href="#">Terms and Conditions</a>.
				          </label>
				        </div>
				    </div>
				    <div class="form-group">
				      <div class="col-lg-10">
				        <button id="submit" type="submit" class="col-lg-10 btn btn-primary">Submit</button>
				      </div>
				    </div>
				  </fieldset>
				</form>
				{{# has_error}}
				<div class="col-lg-10">
					<p class="text-danger">{{error}}</p>
				</div>
				{{/ has_error}}
			</div>
	  	</div>
	  </div>
	</div>
</div>
</body>
</html>