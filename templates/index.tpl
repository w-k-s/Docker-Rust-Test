<!doctype html>
<html>
<head>
	<title>{{> title}}</title>
	<link rel="stylesheet" href="static/bootstrap.min.css"></link>
	<link rel="stylesheet" href="static/main.css"></link>
</head>
<body>
{{> nav}}
<div class="jumbotron">
	<div class="container-fluid">
	  <div class="row">
	    <div class="col-lg-6 visible-md-block">.col-md-6</div>
	  	<div class="col-lg-6">
	  		<div>
				<form name="register" class="form-horizontal" method="post" action="/register">
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
				        <input type="text" class="form-control" id="inputEmail" placeholder="Email" name="email">
				      </div>
				    </div>
				    <div class="form-group ">
				      <label for="inputPassword" class="col-lg-8 control-label">Password</label>
				      <div class="col-lg-8">
				        <input type="password" class="form-control" id="inputPassword" placeholder="Password" name="password">
				        <div class="checkbox">
				          <label>
				            <input type="checkbox">I agree to the <a href="#">Terms and Conditions</a>.
				          </label>
				        </div>
				      </div>
				    </div>
				    <div class="form-group">
				      <div class="col-lg-10">
				        <button type="submit" class="col-lg-10 btn btn-primary">Submit</button>
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