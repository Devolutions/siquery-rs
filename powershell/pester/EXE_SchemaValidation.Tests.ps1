Import-Module "$PSScriptRoot/../siquery"
$Tables = Get-siqList


Describe '<_> Schema Validation' -ForEach $Tables {
	Context '<Table> executable schema validation' -ForEach @(@{ Table = $_ }) {
		BeforeAll {
			$Schema = Get-SiqSchema $Table
			$SchemaProperties = $Schema
				| Select-Object -Property * -ExcludeProperty TableName
				| Get-Member -MemberType NoteProperty
				| Select-Object -ExpandProperty Name
			$Data = Get-Siq $Table | Select-Object -First 1
		}

		It 'has schema from exe' { $Schema | Should -Not -BeNullOrEmpty }

		Context 'data property <_> is of correct type' -ForEach $SchemaProperties {
			BeforeAll {
				$ItemPropType = $Data."$_".GetType()
				$SchemaPropType = [System.Type]::GetType($Schema."$_")
			}
			It 'validates schema' {
				$ItemPropType | Should -eq $SchemaPropType -Because "executable's schema says $Table $_ should be of type $($SchemaPropType.FullName)"
				Write-Host "`n`n"
			}
		}
	}
}
