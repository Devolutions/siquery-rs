function ConvertTo-PascalCase {
    [OutputType('System.String')]
    param(
        [Parameter(Position = 0)]
        [string] $Value
    )

    # https://devblogs.microsoft.com/oldnewthing/20190909-00/?p=102844
    return [regex]::replace($Value.ToLower(), '(^|_)(.)', { $args[0].Groups[2].Value.ToUpper() })
}
