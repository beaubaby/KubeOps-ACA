Protocol การสร่้าง Webservices
- SOAP (ERP - Embed system in one platform - ครบวงจร) call via XML (to transfer data)
- RestAPI 
- GRPC (Streaming video)

website -> inspect (F12) -> developer tool (network)
transfer data via document type

RESTAPI
- GET : รับ
- POST : รับส่ง + เข้ารหัส
- DELETE
- PUT : UPDATE 

Work of web servicess
Client - ฝั่งผู้ใช้งาน (User)
Server - เจ้าของข้อมูล เช่น google.com (เจ้า่ของคือ google)

How to use (Request method)
www.google.com
         (package = data)
Client -> Request -> Server
|      <- Response      |
Users               google


Request  = user request (ขาไป)
Response = server response (ขากลับ)

in Request and response will have Header and Body

Request/Response strategy
็HTTP method and status
- HTTP Status
4xx
5xx
2xx

- HTTP Method
- GET : รับ
- POST : รับส่ง + เข้ารหัส = บันทึกข้อมูล ค้นหาข้อมูลเข้ารหัส
- DELETE
- PUT : UPDATE 

Load balancer = จะไกด์  user ไปหา server ซึ่งอยู่ฝั่ง server 
ถ้า LB รับ traffic ไม่ไหว จะโชว์ (502 Bad gateway)
ถ้า LB มี set ไว้ limit response time >1 min ถ้าเกิน จะ return method (504 gateway timeout)

EX. 
- website เรียนออนไลน์ต้องทำอะไร 
- ทำอย่างไร (Design ผลลัพธ์ของ result สิ่งที่เราจะสร้าง)