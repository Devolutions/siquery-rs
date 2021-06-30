Import-Module "$PSScriptRoot/../siquery"

Describe 'logical_drives table' {
	InModuleScope siquery {
		Context 'schema validation' {
			BeforeAll {
				$SkipTableTest = (-Not $IsWindows) ? $true : $false
				$allSchemas = Get-ChildItem "$PSScriptRoot/../siquery/schema/*.json"
			}

			It 'has a schema' {
				$logical_drives = $allSchemas | Where-Object { $_.BaseName -eq 'logical_drives' }
				$logical_drives | Should -Not -BeNullOrEmpty
			}

			# I want to nest these as dependent tests but am unsure on the right syntax.
			It 'has a valid schema' {
				$logical_drives = $allSchemas | Where-Object { $_.BaseName -eq 'logical_drives' }
				$schema = Get-Content $logical_drives.FullName -Raw
				$schema | Test-Json | Should -Be $true
			}

			# Windows Only Table -
			It 'validates schema' -Skip:$SkipTableTest {
				$logical_drives = Get-Siq 'logical_drives' | Select-Object -First 1

				$schema = Get-Content ($allSchemas | Where-Object { $_.BaseName -eq 'logical_drives' }).FullName -Raw

				$json = $logical_drives | ConvertTo-Json -ErrorAction Stop
				$json | Test-Json -Schema $schema | Should -Be $true
			}
		}
	}
}
