Invoke-WebRequest -Uri "https://mein.fitnessfirst.de/api/magicline/openapi/classes/hamburg3/booking/book" `
  -Method Post `
  -Headers @{"Cookie" = "PHPSESSID=8htjsui4lh90gqdinq89i45vus" } `
  -ContentType "application/json" `
  -Body "{`"customerId`":`"1380798137`",`"classSlotId`":1486092405,`"classId`":1355292810,`"clubId`":`"hamburg3`",`"clubName`":`"Hamburg - Eppendorf`",`"className`":`"Hyrox (M/F)`"}"
