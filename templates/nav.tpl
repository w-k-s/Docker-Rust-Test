<nav class="navbar navbar-toggleable-md navbar-inverse bg-inverse">
  <button class="navbar-toggler navbar-toggler-right" type="button" data-toggle="collapse" data-target="#navbarColor01" aria-controls="navbarColor01" aria-expanded="false" aria-label="Toggle navigation">
    <span class="navbar-toggler-icon"></span>
  </button>
  <a class="navbar-brand" href="#">{{> title}}</a>
  <div class="collapse navbar-collapse" id="navbarColor01">
    <ul class="navbar-nav mr-auto">
      <li class="nav-item active">
        <a class="nav-link" href="#">Home<span class="sr-only">(current)</span></a>
      </li>
      <li class="nav-item">
        <a class="nav-link" href="#">Features</a>
      </li>
      <li class="nav-item">
        <a class="nav-link" href="#">About</a>
      </li>
    </ul>
    {{#signed_in}}
      <p>Signed-in</p>
    {{/signed_in}}
    {{^signed_in}}
    <!--<a href="login" class="nav-link">Sign-in</a><span class="nav-item"> or </span><a href="/register" class="nav-link">Sign-up</a>-->
     <form class="navbar-form navbar-right" style="display:inline-block">
        <div class="form-group" style="display:inline-block">
          <div style="color:white">Username</div>
          <input type="text" name="username" class="form-control" placeholder="Username">
        </div>
        <div class="form-group" style="display:inline-block" >
          <div style="color:white">Password</div>
          <input type="password" name="password" class="form-control" placeholder="Password">
        </div>
        <button type="submit" class="btn btn-default" style="display:inline-block">Submit</button>
      </form>
    {{/signed_in}}
  </div>
</nav>