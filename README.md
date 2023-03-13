# LapkaDB
Experimental DB to learn core concepts

# Концепция 
- append-only
- бд состоит из набора коллекций (ака таблиц)
- каждая коллекция имеет свою структуру (схему)
- клиент передает и получает данные в json'e
- git-like версионирование данных и структур коллекций
- взаимодействие происходит по http

# Структура файлов таблиц 
- файл коллекции разбит на страницы фиксированного размера
- страница начинается с метаданных которые растут вверх и данных, которые растут сверху вниз по адресному пространству
- в метаданных содержится битмапа с указателями на null для nullable полей в схеме
- в метаданных есть указатель на коммит при котором он сделан
- странца не может превышать свой установленный размер
- [в случае если запись не влазит в страницу](https://wiki.postgresql.org/wiki/TOAST) + указание в метаданных в странице
- у каждого типа данных кроме строк и массивов в схеме есть фиксированный размер в байтах
- у каждого поля в схеме объекта есть порядковый номер по которому он будет уложен на диск
- для строк и массивов сначала идут указатели на окончание, потом данные
- вложенные объекты укладываются в "плоскую структуру" т.е. 
```json
{
    "id":1,
    "user":{
        "name":"pepega",
        "age":13
    }
}
```
превращается в 
```
| id | user.name | user.age |
```
- объекты в массиве укладываются по общим принципам, последовательно и безразрывно

# Миграции и "изменения" данных
- миграция может только добавлять необязательные поля
- возможность "изменить" данные путем добавления новой (специальной) записи, в которой можно игнорировать любые обязательные поля - служит как "патч" к уже существующим данным и соответственно возможность выгрузки данных с учетом этих "патчей"