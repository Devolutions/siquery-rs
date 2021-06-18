Function Get-SiqSchema {
	[CmdletBinding()]
	param(
		[String] $Table = '*'
	)

	Begin {
		$Schema = [System.Collections.Generic.List[Object]]::New()
		$siq_exe = Find-SiqExecutable
	}
	Process {
		$SchemaCall = ($Table -ne '*') ? (& "$siq_exe" --schema $Table) : (& "$siq_exe" --schema)
		if ($SchemaCall -inotlike 'no such table ') {
			Foreach ($Line in $SchemaCall) {
				$ConvertedTableSchema = [Ordered]@{}
				$Line = $Line -Replace '^CREATE TABLE ', ''
				$TableName = $Line.Split('(')[0]
				$ConvertedTableSchema.Add('TableName', $TableName)
				$TableFields = $Line.Split('(')[1].TrimEnd(');')
				Foreach ($Field in $TableFields.Split(',').Trim()) {
					$FieldSplit = $Field.Split('" ')
					$FieldName = $FieldSplit[0].TrimStart('"').Trim()
					$FieldType = $FieldSplit[1].Trim()
					$Type = switch ($FieldType) {
						'INTEGER' { '[int32]' }
						'TEXT' { '[string]' }
						Default {
							Write-Host "`$_: $_"
							$TypeCheck = Get-TypeData -TypeName "*$FieldType*"
							if ($TypeCheck.Count -ne 1) {
								Write-Error "Unable to find type $FieldType for field $FieldName on table $TableName. Defaulting to [String]."
								'[string]' #Default to string if we cant determine type.
							} else {
								"[$($TypeCheck.TypeName)]"
							}
						}
					}
					$ConvertedTableSchema.Add($FieldName, $Type)
				}
				$Schema.Add($ConvertedTableSchema)
			}
		} else {
			$Schema.Add([Ordered]@{'TableName' = $SchemaCall })
		}
	}
	End { Return $Schema }
}
