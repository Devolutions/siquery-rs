function ConvertTo-SnakeCase {
	[OutputType('System.String')]
	param(
		[Parameter(Position = 0)]
		[string] $Value
	)

	return [regex]::replace($Value, '(?<=.)(?=[A-Z])', '_').ToLower()
}
