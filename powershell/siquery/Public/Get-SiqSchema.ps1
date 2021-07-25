using namespace System.Management.Automation
using namespace System.Collections.Generic

Function Get-SiqSchema {
	[CmdletBinding()]
	param(
		[Parameter( Position = 0, ValueFromPipeline )]
		[Alias('Name', 'Table')]
		[string]$TableName = '*'
	)

	Begin {
		$Schema = [List[Object]]::New()
		$siq_exe = Find-SiqExecutable
	}

	Process {
		$SchemaCall = ($Table -ne '*') ? (& "$siq_exe" --schema $TableName) : (& "$siq_exe" --schema)
		if ($SchemaCall -inotlike 'no such table ') {
			Foreach ($Line in $SchemaCall) {
				$ConvertedTableSchema = [PSCustomObject]@{}

				$Line = $Line -Replace '^CREATE TABLE ', ''
				$SchemaTableName = $Line.Split('(')[0]
				$ConvertedTableSchema.PSObject.Properties.Add([PSNoteProperty]::New('TableName', $SchemaTableName))

				$TableFields = $Line.Split('(')[1].TrimEnd(');')
				Foreach ($Field in $TableFields.Split(',').Trim()) {
					$FieldSplit = $Field.Split('" ')
					$FieldName = $FieldSplit[0].TrimStart('"').Trim()

					# Determine Field Type
					$Type = switch ($FieldSplit[1].Trim()) {
						'INTEGER' { 'System.Int32' }
						'TEXT' { 'System.String' }
						Default {
							Write-Host "`$_: $_"
							$TypeCheck = Get-TypeData -TypeName "*$_*"
							if ($TypeCheck.Count -ne 1) {
								Write-Error "Unable to find type '$_' for field $FieldName on table $SchemaTableName. Defaulting to [string]."
								'System.String' #Default to string if we cant determine type.
							} else {
								"$($TypeCheck.TypeName)"
							}
						}
					}

					$ConvertedTableSchema.PSObject.Properties.Add([PSNoteProperty]::New($FieldName, $Type))
				}

				$Schema.Add($ConvertedTableSchema)
			}
		} else {
			$Schema.Add([PSCustomObject]@{'TableName' = $SchemaCall })
		}
	}

	End { Return $Schema }
}
