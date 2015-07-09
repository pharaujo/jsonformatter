import Foundation

func JSONStringify(value: AnyObject,prettyPrinted:Bool = false) -> String {
    let options = prettyPrinted ? NSJSONWritingOptions.PrettyPrinted : NSJSONWritingOptions(rawValue: 0)
    if NSJSONSerialization.isValidJSONObject(value) {
        let data = NSJSONSerialization.dataWithJSONObject(value, options: options, error: nil)
        if let string = NSString(data: data!, encoding: NSUTF8StringEncoding) {
            return string as String
        }
    }
    return ""
}

var jsonData: NSData?

switch Process.argc {
case 1:
    var stdin = NSFileHandle.fileHandleWithStandardInput()
    jsonData = stdin.availableData
case 2:
    let filepath:String? = String.fromCString(Process.arguments[1])
    jsonData = NSData(contentsOfFile: filepath!, options: .DataReadingMappedIfSafe, error: nil)
default:
    exit(1)
}

let jsonObject : AnyObject! = NSJSONSerialization.JSONObjectWithData(jsonData!, options: nil, error: nil)
print(JSONStringify(jsonObject, prettyPrinted: true))