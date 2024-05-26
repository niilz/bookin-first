Invoke-WebRequest -Uri "https://mein.fitnessfirst.de/api/magicline/openapi/classes/hamburg3/booking/1518353341" `
  -Method Delete `
  -Headers @{"Cookie" = "PHPSESSID=p0hf2ngfjlbi05q5namm15g52b" } `
  -ContentType "application/json" `
  -Body "{`"customerId`":`"1380798137`",`"classSlotId`":1486084677}"
