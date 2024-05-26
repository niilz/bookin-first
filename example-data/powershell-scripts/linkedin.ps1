$session = New-Object Microsoft.PowerShell.Commands.WebRequestSession
$session.UserAgent = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36 Edg/120.0.0.0"
$session.Cookies.Add((New-Object System.Net.Cookie("bcookie", "`"v=2&2d9222b5-93b2-47d1-8984-aadc07b93651`"", "/", ".linkedin.com")))
$session.Cookies.Add((New-Object System.Net.Cookie("li_alerts", "e30=", "/", "www.linkedin.com")))
$session.Cookies.Add((New-Object System.Net.Cookie("bscookie", "`"v=1&202311200823057bae3ef1-bce1-4368-8f13-0d7dd6c6fb34AQHk-bCPdfxia-Cr4ECqTBfrEKRWm2kc`"", "/", ".www.linkedin.com")))
$session.Cookies.Add((New-Object System.Net.Cookie("lls-integration", "AQH2wKPPfwoQOwAAAYvr0-Kx2c7dE-_FkXbrWVUvjEFRro0mOeoToLnuZ-UC-uNpiwU2y3MVULXb3C6XHYID1YkZzsja88FzlVi_ndFwWJDJqSP7edJLQsZXmOGOV01ugsaNO797tuY7ujSOQbMpUYWmykp3hxkjwAg19ag_NMOg9vJ-u9BwkkZde3yyK3HNJiLXuuFH9YTZV3zA7sqaOhhfcqEbryTxnXKD", "/", "www.linkedin.com")))
$session.Cookies.Add((New-Object System.Net.Cookie("li_rm", "AQEqwuxvHHOlagAAAYz94Q2hLRN2jna3ix3rEHP331BkHTJwV-yJzOaMxx4T6cns3SwW4X6v70z-Ow8TjBYNtph1XDjNgAK3WGsb4p4uLI_SqcKj59E7ZZAP", "/", ".www.linkedin.com")))
$session.Cookies.Add((New-Object System.Net.Cookie("li_theme", "light", "/", ".www.linkedin.com")))
$session.Cookies.Add((New-Object System.Net.Cookie("li_theme_set", "app", "/", ".www.linkedin.com")))
$session.Cookies.Add((New-Object System.Net.Cookie("timezone", "Europe/Berlin", "/", ".www.linkedin.com")))
$session.Cookies.Add((New-Object System.Net.Cookie("dfpfpt", "05c3d83df3ca4b94a30fd7fbe75faefa", "/", ".linkedin.com")))
$session.Cookies.Add((New-Object System.Net.Cookie("fptctx2", "taBcrIH61PuCVH7eNCyH0MJojnuUODHcZ6x9WoxhgClqG%252fFgJjxquq17SNf8vfWPFxt%252b82%252fr49AIHXB9aijzeSrXiaWjnrIbksasKWiL17ouc3avZ1IN6ndkEgtqwzStu9JchCwNDI1l%252b0vxv3icYikqyVjO2SeFbbWPQsysXXHGaO1aFCMP5DkpIomkxM6WvKA8xZowTT2iqoxwixM2VFiCnqDgSTO%252bTggS4w%252fko8Nh8Fx3E%252fnBPKVEThD4SDVpci%252fi9nkxX2A2gar8Phl3dfHyA2ffTDimLV6UuBCwQlb%252f2K4NnpHWyY95QjeF4bO5nEcozCIgNN%252fHx7Z98NYzSmcEHk1i2b8qeOXShVp9v24%253d", "/", ".linkedin.com")))
$session.Cookies.Add((New-Object System.Net.Cookie("li_g_recent_logout", "v=1&true", "/", "www.linkedin.com")))
$session.Cookies.Add((New-Object System.Net.Cookie("visit", "v=1&M", "/", ".linkedin.com")))
$session.Cookies.Add((New-Object System.Net.Cookie("lang", "v=2&lang=de-de", "/", ".linkedin.com")))
$session.Cookies.Add((New-Object System.Net.Cookie("g_state", "{`"i_p`":1706884173078,`"i_l`":2}", "/", "www.linkedin.com")))
$session.Cookies.Add((New-Object System.Net.Cookie("li_gc", "MTs0MjsxNzA2Nzk3Nzc2OzI7MDIxR4lAEd/bonYqy7McS4BYh8P1QO97a9m9FhugxFeurOY=", "/", ".linkedin.com")))
$session.Cookies.Add((New-Object System.Net.Cookie("li_at", "AQEDAS1gW_UC4zu6AAABjWUVFOAAAAGNiSGY4E4Aiy5U9PAWuPZrsJ775E5SvPzlE2zlNwmVL8RHN37jHUKjptzrpnYOqtF39fPvYFx2ZX278PVmrtiN9Psf0f_A0RXbq1b4goNXnYpLdTPzMYO_wtmh", "/", ".www.linkedin.com")))
$session.Cookies.Add((New-Object System.Net.Cookie("liap", "true", "/", ".linkedin.com")))
$session.Cookies.Add((New-Object System.Net.Cookie("JSESSIONID", "`"ajax:0730208152121133684`"", "/", ".www.linkedin.com")))
$session.Cookies.Add((New-Object System.Net.Cookie("AMCVS_14215E3D5995C57C0A495C55%40AdobeOrg", "1", "/", ".linkedin.com")))
$session.Cookies.Add((New-Object System.Net.Cookie("AMCV_14215E3D5995C57C0A495C55%40AdobeOrg", "-637568504%7CMCIDTS%7C19755%7CMCMID%7C87679473384407519712060538820570270882%7CMCAAMLH-1707402702%7C6%7CMCAAMB-1707402702%7C6G1ynYcLPuiQxYZrsz_pkqfLG9yMXBpb2zX5dvJdYQJzPXImdj0y%7CMCOPTOUT-1706805102s%7CNONE%7CMCCIDH%7C-1296379154%7CvVersion%7C5.1.1", "/", ".linkedin.com")))
$session.Cookies.Add((New-Object System.Net.Cookie("aam_uuid", "87463231942141728402042292405831253865", "/", ".linkedin.com")))
$session.Cookies.Add((New-Object System.Net.Cookie("li_mc", "MTsyMTsxNzA2Nzk4NDk5OzI7MDIxNNFI2TyuRwQO66svoUh83FiQMnJTEJxA3g5LoaeO2E0=", "/", ".linkedin.com")))
$session.Cookies.Add((New-Object System.Net.Cookie("lidc", "`"b=TB17:s=T:r=T:a=T:p=T:g=13155:u=1071:x=1:i=1706798953:t=1706885353:v=2:sig=AQGtsvx5awEJk5fLgtvsFlkZ0QWd5WMe`"", "/", ".linkedin.com")))
Invoke-WebRequest -UseBasicParsing -Uri "https://www.linkedin.com/voyager/api/graphql?includeWebMetadata=true&variables=(memberIdentity:ACoAAC1gW_UBtBj9RV0xrKitwFMhY4qw9RLI7og)&queryId=voyagerIdentityDashProfiles.b5c27c04968c409fc0ed3546575b9b7a" `
  -WebSession $session `
  -Headers @{
  "authority"                 = "www.linkedin.com"
  "method"                    = "GET"
  "path"                      = "/voyager/api/graphql?includeWebMetadata=true&variables=(memberIdentity:ACoAAC1gW_UBtBj9RV0xrKitwFMhY4qw9RLI7og)&queryId=voyagerIdentityDashProfiles.b5c27c04968c409fc0ed3546575b9b7a"
  "scheme"                    = "https"
  "accept"                    = "application/vnd.linkedin.normalized+json+2.1"
  "accept-encoding"           = "gzip, deflate, br"
  "accept-language"           = "de,de-DE;q=0.9,en;q=0.8,en-GB;q=0.7,en-US;q=0.6"
  "cache-control"             = "no-cache"
  "csrf-token"                = "ajax:0730208152121133684"
  "dnt"                       = "1"
  "pragma"                    = "no-cache"
  "referer"                   = "https://www.linkedin.com/profile/edit/topcard/"
  "sec-ch-ua"                 = "`"Not_A Brand`";v=`"8`", `"Chromium`";v=`"120`", `"Microsoft Edge`";v=`"120`""
  "sec-ch-ua-mobile"          = "?0"
  "sec-ch-ua-platform"        = "`"Windows`""
  "sec-fetch-dest"            = "empty"
  "sec-fetch-mode"            = "cors"
  "sec-fetch-site"            = "same-origin"
  "x-li-lang"                 = "de_DE"
  "x-li-page-instance"        = "urn:li:page:email_edit_topcard_redirect;fa91cc1f-d5c5-4b79-974c-8c061a7bf95b"
  "x-li-track"                = "{`"clientVersion`":`"1.13.10131`",`"mpVersion`":`"1.13.10131`",`"osName`":`"web`",`"timezoneOffset`":1,`"timezone`":`"Europe/Berlin`",`"deviceFormFactor`":`"DESKTOP`",`"mpName`":`"voyager-web`",`"displayDensity`":1,`"displayWidth`":1920,`"displayHeight`":1080}"
  "x-restli-protocol-version" = "2.0.0"
}
