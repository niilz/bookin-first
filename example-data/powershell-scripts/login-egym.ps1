echo "Starting to login via egym"
$egymLogin = Invoke-WebRequest -Uri "https://id.egym.com/login" `
  -Method "POST" `
  -ContentType "application/x-www-form-urlencoded; charset=UTF-8" `
  -Body "username=niilz%40outlook.de&password=nnn.VielStatt5chwer&clientId=a175bce7-3e5b-4863-92a1-efc1991ae6fd&callbackUrl=https%3A%2F%2Fwww.fitnessfirst.de%2Fmein-fitnessfirst"

$fitnessFirstBaseUri = "https://mein.fitnessfirst.de"
$egymLoginPath = "$fitnessFirstBaseUri/egymid-login?token="

$token = $egymLogin.content.split("?token=")[1]
echo "retrieved token: $token"

$session = New-Object Microsoft.PowerShell.Commands.WebRequestSession
echo "logging into fitness-first"
$loginUri = "$egymLoginPath$token"
echo "Login-URL: $loginUri"
echo "Loggin in"
$loginResponse = (iwr $loginUri -WebSession $session)

$cookie = $session.Cookies.GetCookies($fitnessFirstBaseUri)
echo "Cookie: $cookie"
echo "PHPSESSID=$($cookie.value)"
